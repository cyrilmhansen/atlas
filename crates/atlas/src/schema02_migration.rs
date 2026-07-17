use std::collections::HashSet;

use serde::Deserialize;
use serde_yaml::{Mapping, Value};

use crate::decision_overlay::{CostMetric, CostRegime};
use crate::registry::{Algorithm, Claim, Implementation, Problem, Registry};

const SPARE_CAPACITY: &str = "state.spare_capacity";
const NONADVERSARIAL_HASHING: &str = "workload.nonadversarial_hash_distribution";

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Registry02 {
    schema_version: String,
    conditions: Vec<Condition02>,
    problems: Vec<Problem>,
    algorithms: Vec<Algorithm02>,
    implementations: Vec<Implementation>,
    executions: Vec<Value>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Condition02 {
    id: String,
    statement: Claim<String>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Algorithm02 {
    id: String,
    solves: String,
    name: Claim<String>,
    requires: Option<Claim<Vec<String>>>,
    stable: Option<Claim<bool>>,
    deterministic: Claim<bool>,
    in_place: Option<Claim<bool>>,
    costs: Vec<Claim<GenericCost>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GenericCost {
    metric: CostMetric,
    regime: CostRegime,
    bound: String,
    requires: Vec<String>,
}

fn migrate(source: &str) -> Result<String, String> {
    let mut document = serde_yaml::from_str::<Value>(source).map_err(|error| error.to_string())?;
    let baseline = serde_yaml::from_value::<Registry>(document.clone())
        .map_err(|error| format!("invalid schema 0.1 input: {error}"))?;
    let errors = baseline.validate();
    if !errors.is_empty() {
        return Err(format!("invalid schema 0.1 input: {errors:?}"));
    }

    let root = as_mapping_mut(&mut document, "root")?;
    root.insert(key("schema_version"), Value::String("0.2-draft".into()));
    root.insert(key("conditions"), parse_conditions()?);
    let algorithms = root
        .get_mut(key("algorithms"))
        .and_then(Value::as_sequence_mut)
        .ok_or_else(|| "algorithms must be a sequence".to_string())?;

    for algorithm in algorithms {
        let mapping = as_mapping_mut(algorithm, "algorithm")?;
        let id = string_field(mapping, "id")?.to_string();
        let worst = remove_required(mapping, "time_worst", &id)?;
        let expected = mapping.remove(key("time_expected"));
        let auxiliary = remove_required(mapping, "auxiliary_memory", &id)?;
        let mut costs = vec![
            qualified_claim(worst, "time", "worst", &[])?,
            qualified_claim(auxiliary, "auxiliary_memory", "worst", &[])?,
        ];
        if let Some(expected) = expected {
            let requirements = if id == "associative_map.hashbrown.insert" {
                &[NONADVERSARIAL_HASHING][..]
            } else {
                &[]
            };
            costs.push(qualified_claim(expected, "time", "expected", requirements)?);
        }
        if matches!(
            id.as_str(),
            "priority_queue.binary_heap.push" | "priority_queue.quaternary_heap.push"
        ) {
            costs.extend(parse_heap_profiles()?);
        }
        mapping.insert(key("costs"), Value::Sequence(costs));
    }

    serde_yaml::to_string(&document).map_err(|error| error.to_string())
}

fn validate_draft(contents: &str) -> Result<Registry02, String> {
    let registry =
        serde_yaml::from_str::<Registry02>(contents).map_err(|error| error.to_string())?;
    if registry.schema_version != "0.2-draft" {
        return Err(format!(
            "unexpected schema version {:?}",
            registry.schema_version
        ));
    }
    let mut condition_ids = HashSet::new();
    for condition in &registry.conditions {
        if !condition_ids.insert(condition.id.as_str()) {
            return Err(format!("duplicate condition {:?}", condition.id));
        }
        if condition.statement.value.trim().is_empty()
            || condition.statement.source.trim().is_empty()
        {
            return Err(format!("incomplete condition {:?}", condition.id));
        }
    }
    for algorithm in &registry.algorithms {
        if algorithm.costs.is_empty() {
            return Err(format!("algorithm {:?} has no cost profile", algorithm.id));
        }
        let mut profiles = HashSet::new();
        for claim in &algorithm.costs {
            let profile = &claim.value;
            for condition in &profile.requires {
                if !condition_ids.contains(condition.as_str()) {
                    return Err(format!(
                        "algorithm {:?} references unknown condition {:?}",
                        algorithm.id, condition
                    ));
                }
            }
            if claim.source.trim().is_empty() || profile.bound.trim().is_empty() {
                return Err(format!(
                    "algorithm {:?} has an incomplete cost",
                    algorithm.id
                ));
            }
            let identity = (
                profile.metric,
                profile.regime,
                profile.bound.as_str(),
                profile.requires.as_slice(),
            );
            if !profiles.insert(identity) {
                return Err(format!("algorithm {:?} has a duplicate cost", algorithm.id));
            }
        }
    }
    Ok(registry)
}

fn qualified_claim(
    mut claim: Value,
    metric: &str,
    regime: &str,
    requires: &[&str],
) -> Result<Value, String> {
    let mapping = as_mapping_mut(&mut claim, "claim")?;
    let bound = mapping
        .remove(key("value"))
        .ok_or_else(|| "claim is missing value".to_string())?;
    let mut value = Mapping::new();
    value.insert(key("metric"), Value::String(metric.into()));
    value.insert(key("regime"), Value::String(regime.into()));
    value.insert(key("bound"), bound);
    value.insert(
        key("requires"),
        Value::Sequence(
            requires
                .iter()
                .map(|condition| Value::String((*condition).into()))
                .collect(),
        ),
    );
    mapping.insert(key("value"), Value::Mapping(value));
    Ok(claim)
}

fn parse_conditions() -> Result<Value, String> {
    serde_yaml::from_str(
        r#"
- id: state.spare_capacity
  statement:
    value: the destination state can accept the operation without growth
    level: declared
    source: definition:state.spare_capacity
- id: workload.nonadversarial_hash_distribution
  statement:
    value: the supplied hash distribution is not adversarial for the implementation
    level: declared
    source: definition:workload.nonadversarial_hash_distribution
"#,
    )
    .map_err(|error| error.to_string())
}

fn parse_heap_profiles() -> Result<Vec<Value>, String> {
    serde_yaml::from_str(
        r#"
- value:
    metric: time
    regime: worst
    bound: O(log n)
    requires: [state.spare_capacity]
  level: inferred
  source: docs:phase2/k-m4-dual-import-comparison.md
- value:
    metric: allocation
    regime: worst
    bound: none
    requires: [state.spare_capacity]
  level: inferred
  source: docs:phase2/k-m5-heap-condition-result.md
"#,
    )
    .map_err(|error| error.to_string())
}

fn remove_required(mapping: &mut Mapping, field: &str, id: &str) -> Result<Value, String> {
    mapping
        .remove(key(field))
        .ok_or_else(|| format!("algorithm {id:?} is missing {field}"))
}

fn string_field<'a>(mapping: &'a Mapping, field: &str) -> Result<&'a str, String> {
    mapping
        .get(key(field))
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{field} must be a string"))
}

fn as_mapping_mut<'a>(value: &'a mut Value, context: &str) -> Result<&'a mut Mapping, String> {
    value
        .as_mapping_mut()
        .ok_or_else(|| format!("{context} must be a mapping"))
}

fn key(value: &str) -> Value {
    Value::String(value.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema02_dry_run_is_deterministic_typed_and_lossless() {
        let source = include_str!("../../../registry/atlas.yaml");
        let first = migrate(source).unwrap();
        let second = migrate(source).unwrap();
        assert_eq!(first, second);

        let baseline = serde_yaml::from_str::<Registry>(source).unwrap();
        let migrated = validate_draft(&first).unwrap();
        assert_eq!(migrated.problems.len(), baseline.problems.len());
        assert_eq!(migrated.algorithms.len(), baseline.algorithms.len());
        assert_eq!(
            migrated.implementations.len(),
            baseline.implementations.len()
        );
        assert_eq!(migrated.executions.len(), baseline.executions.len());

        for original in &baseline.algorithms {
            let draft = migrated
                .algorithms
                .iter()
                .find(|algorithm| algorithm.id == original.id)
                .unwrap();
            assert_claim_migrated(original, draft, CostMetric::Time, CostRegime::Worst);
            assert_claim_migrated(
                original,
                draft,
                CostMetric::AuxiliaryMemory,
                CostRegime::Worst,
            );
            if original.time_expected.is_some() {
                assert_claim_migrated(original, draft, CostMetric::Time, CostRegime::Expected);
            }
        }

        for id in [
            "priority_queue.binary_heap.push",
            "priority_queue.quaternary_heap.push",
        ] {
            let algorithm = algorithm(&migrated, id);
            assert!(has_cost(
                algorithm,
                CostMetric::Time,
                CostRegime::Worst,
                "O(log n)",
                &[SPARE_CAPACITY]
            ));
            assert!(has_cost(
                algorithm,
                CostMetric::Allocation,
                CostRegime::Worst,
                "none",
                &[SPARE_CAPACITY]
            ));
        }
        assert!(has_cost(
            algorithm(&migrated, "associative_map.hashbrown.insert"),
            CostMetric::Time,
            CostRegime::Expected,
            "O(1)",
            &[NONADVERSARIAL_HASHING]
        ));
    }

    #[test]
    fn schema02_draft_rejects_invalid_cost_knowledge() {
        let source = include_str!("../../../registry/atlas.yaml");

        let mut unknown = migrated_value(source);
        let costs = algorithm_costs(&mut unknown, "priority_queue.binary_heap.push");
        costs.last_mut().unwrap()["value"]["requires"] =
            Value::Sequence(vec![Value::String("state.missing".into())]);
        assert!(draft_error(unknown).contains("unknown condition"));

        let mut duplicate = migrated_value(source);
        let costs = algorithm_costs(&mut duplicate, "sort.merge.top_down");
        costs.push(costs[0].clone());
        assert!(draft_error(duplicate).contains("duplicate cost"));

        let mut incomplete = migrated_value(source);
        algorithm_costs(&mut incomplete, "sort.merge.top_down")[0]["source"] =
            Value::String(String::new());
        assert!(draft_error(incomplete).contains("incomplete cost"));
    }

    fn assert_claim_migrated(
        original: &Algorithm,
        migrated: &Algorithm02,
        metric: CostMetric,
        regime: CostRegime,
    ) {
        let original_claim = match (metric, regime) {
            (CostMetric::Time, CostRegime::Worst) => &original.time_worst,
            (CostMetric::Time, CostRegime::Expected) => original.time_expected.as_ref().unwrap(),
            (CostMetric::AuxiliaryMemory, CostRegime::Worst) => &original.auxiliary_memory,
            _ => unreachable!(),
        };
        let expected_conditions = if original.id == "associative_map.hashbrown.insert"
            && regime == CostRegime::Expected
        {
            &[NONADVERSARIAL_HASHING][..]
        } else {
            &[]
        };
        let migrated_claim = migrated
            .costs
            .iter()
            .find(|claim| {
                claim.value.metric == metric
                    && claim.value.regime == regime
                    && claim.value.bound == original_claim.value
                    && claim.value.requires == expected_conditions
            })
            .unwrap_or_else(|| panic!("missing migrated claim for {}", original.id));
        assert_eq!(
            migrated_claim.level.to_string(),
            original_claim.level.to_string()
        );
        assert_eq!(migrated_claim.source, original_claim.source);
    }

    fn algorithm<'a>(registry: &'a Registry02, id: &str) -> &'a Algorithm02 {
        registry
            .algorithms
            .iter()
            .find(|algorithm| algorithm.id == id)
            .unwrap()
    }

    fn has_cost(
        algorithm: &Algorithm02,
        metric: CostMetric,
        regime: CostRegime,
        bound: &str,
        requires: &[&str],
    ) -> bool {
        algorithm.costs.iter().any(|claim| {
            claim.value.metric == metric
                && claim.value.regime == regime
                && claim.value.bound == bound
                && claim.value.requires == requires
        })
    }

    fn migrated_value(source: &str) -> Value {
        serde_yaml::from_str(&migrate(source).unwrap()).unwrap()
    }

    fn algorithm_costs<'a>(document: &'a mut Value, id: &str) -> &'a mut Vec<Value> {
        document["algorithms"]
            .as_sequence_mut()
            .unwrap()
            .iter_mut()
            .find(|algorithm| algorithm["id"] == id)
            .unwrap()["costs"]
            .as_sequence_mut()
            .unwrap()
    }

    fn draft_error(document: Value) -> String {
        validate_draft(&serde_yaml::to_string(&document).unwrap())
            .err()
            .expect("invalid fixture must be rejected")
    }
}
