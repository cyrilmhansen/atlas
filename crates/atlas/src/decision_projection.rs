use crate::decision_overlay::{
    Atom, AtomKind, Candidate, CostFact, CostMetric, CostRegime, CostRequirement, DecisionOverlay,
    Evidence, Fact, OverlayEvidenceLevel, Request, SUPPORTED_OVERLAY_VERSION,
};
use crate::registry::{Algorithm, EvidenceLevel, Implementation, Registry};

const CONTRACT: &str = "capability.problem_contract";
const ALLOCATES: &str = "effect.allocates_storage";
const REQUEST_ID: &str = "request.consumer.exact_without_allocation";
const SPARE_CAPACITY: &str = "condition.spare_capacity";
const CONDITIONED_COST_REQUEST_ID: &str = "request.consumer.log_push_with_spare_capacity";

pub(crate) fn project_exact_without_allocation(
    registry: &Registry,
    problem_id: &str,
) -> Result<DecisionOverlay, String> {
    let candidates = candidates_for_problem(registry, problem_id)?
        .into_iter()
        .map(|(implementation, algorithm)| {
            let allocation = (implementation.effects.value.allocation != "none").then(|| Fact {
                atom: ALLOCATES.into(),
                evidence: evidence(implementation.effects.level, &implementation.effects.source),
            });
            Candidate {
                id: implementation.id.clone(),
                source: format!("registry:{}", implementation.id),
                provides: vec![Fact {
                    atom: CONTRACT.into(),
                    evidence: Evidence {
                        level: OverlayEvidenceLevel::Declared,
                        source: format!("registry:{}", algorithm.id),
                        proof: None,
                    },
                }],
                requires: Vec::new(),
                guarantees: Vec::new(),
                effects: allocation.into_iter().collect(),
                consumes_state: None,
                produces_state: None,
                costs: Vec::new(),
            }
        })
        .collect();

    let overlay = DecisionOverlay {
        overlay_version: SUPPORTED_OVERLAY_VERSION.into(),
        atoms: vec![
            Atom {
                id: CONTRACT.into(),
                kind: AtomKind::Capability,
            },
            Atom {
                id: ALLOCATES.into(),
                kind: AtomKind::Effect,
            },
        ],
        candidates,
        relations: Vec::new(),
        equivalences: Vec::new(),
        requests: vec![Request {
            id: REQUEST_ID.into(),
            accepts: CONTRACT.into(),
            provides_conditions: Vec::new(),
            requires_guarantees: Vec::new(),
            forbids_effects: vec![ALLOCATES.into()],
            consumes_state: None,
            maximum_costs: Vec::new(),
            accepted_evidence: vec![
                OverlayEvidenceLevel::Declared,
                OverlayEvidenceLevel::Inferred,
                OverlayEvidenceLevel::Tested,
                OverlayEvidenceLevel::Observed,
                OverlayEvidenceLevel::Proven,
            ],
        }],
    };
    let errors = overlay.validate();
    if errors.is_empty() {
        Ok(overlay)
    } else {
        Err(errors
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("; "))
    }
}

pub(crate) fn project_conditioned_worst_time(
    registry: &Registry,
    problem_id: &str,
) -> Result<DecisionOverlay, String> {
    let candidates = candidates_for_problem(registry, problem_id)?
        .into_iter()
        .map(|(implementation, algorithm)| Candidate {
            id: implementation.id.clone(),
            source: format!("registry:{}", implementation.id),
            provides: vec![Fact {
                atom: CONTRACT.into(),
                evidence: Evidence {
                    level: OverlayEvidenceLevel::Declared,
                    source: format!("registry:{}", algorithm.id),
                    proof: None,
                },
            }],
            requires: Vec::new(),
            guarantees: Vec::new(),
            effects: Vec::new(),
            consumes_state: None,
            produces_state: None,
            costs: vec![CostFact {
                operation: "push".into(),
                metric: CostMetric::Time,
                regime: CostRegime::Worst,
                bound: algorithm.time_worst.value.clone(),
                requires: Vec::new(),
                evidence: evidence(algorithm.time_worst.level, &algorithm.time_worst.source),
            }],
        })
        .collect();

    let overlay = DecisionOverlay {
        overlay_version: SUPPORTED_OVERLAY_VERSION.into(),
        atoms: vec![
            Atom {
                id: CONTRACT.into(),
                kind: AtomKind::Capability,
            },
            Atom {
                id: SPARE_CAPACITY.into(),
                kind: AtomKind::Condition,
            },
        ],
        candidates,
        relations: Vec::new(),
        equivalences: Vec::new(),
        requests: vec![Request {
            id: CONDITIONED_COST_REQUEST_ID.into(),
            accepts: CONTRACT.into(),
            provides_conditions: vec![SPARE_CAPACITY.into()],
            requires_guarantees: Vec::new(),
            forbids_effects: Vec::new(),
            consumes_state: None,
            maximum_costs: vec![CostRequirement {
                operation: "push".into(),
                metric: CostMetric::Time,
                regime: CostRegime::Worst,
                bound: "O(log n)".into(),
                requires: vec![SPARE_CAPACITY.into()],
            }],
            accepted_evidence: vec![
                OverlayEvidenceLevel::Declared,
                OverlayEvidenceLevel::Inferred,
                OverlayEvidenceLevel::Tested,
                OverlayEvidenceLevel::Observed,
                OverlayEvidenceLevel::Proven,
            ],
        }],
    };
    let errors = overlay.validate();
    if errors.is_empty() {
        Ok(overlay)
    } else {
        Err(errors
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("; "))
    }
}

pub(crate) fn consumer_request_id() -> &'static str {
    REQUEST_ID
}

pub(crate) fn conditioned_cost_request_id() -> &'static str {
    CONDITIONED_COST_REQUEST_ID
}

fn candidates_for_problem<'a>(
    registry: &'a Registry,
    problem_id: &str,
) -> Result<Vec<(&'a Implementation, &'a Algorithm)>, String> {
    if !registry
        .problems
        .iter()
        .any(|problem| problem.id == problem_id)
    {
        return Err(format!("unknown problem {problem_id:?}"));
    }
    let algorithms = registry
        .algorithms
        .iter()
        .filter(|algorithm| algorithm.solves == problem_id)
        .collect::<Vec<_>>();
    Ok(registry
        .implementations
        .iter()
        .filter_map(|implementation| {
            algorithms
                .iter()
                .find(|algorithm| algorithm.id == implementation.implements)
                .map(|algorithm| (implementation, *algorithm))
        })
        .collect())
}

fn evidence(level: EvidenceLevel, source: &str) -> Evidence {
    Evidence {
        level: match level {
            EvidenceLevel::Declared => OverlayEvidenceLevel::Declared,
            EvidenceLevel::Inferred => OverlayEvidenceLevel::Inferred,
            EvidenceLevel::Tested => OverlayEvidenceLevel::Tested,
            EvidenceLevel::Observed => OverlayEvidenceLevel::Observed,
            EvidenceLevel::Proven => OverlayEvidenceLevel::Proven,
        },
        source: source.into(),
        proof: None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

    use super::*;
    use crate::decision_evaluator::evaluate_request;
    use crate::registry::load_registry;

    fn caller_storage_top_k(values: &[i32], k: usize, output: &mut Vec<i32>) {
        assert!(output.capacity() >= k);
        output.clear();
        for &value in values {
            let position = output.partition_point(|retained| *retained >= value);
            if output.len() < k {
                output.insert(position, value);
            } else if position < k {
                output.pop();
                output.insert(position, value);
            }
        }
    }

    #[test]
    fn consumer_projection_discovers_and_rejects_allocating_top_k_candidates() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let registry = load_registry(&workspace.join("registry/atlas.yaml")).unwrap();
        let overlay = project_exact_without_allocation(&registry, "stream.top_k").unwrap();
        let decisions = evaluate_request(&overlay, consumer_request_id()).unwrap();

        let algorithms = registry
            .algorithms
            .iter()
            .filter(|algorithm| algorithm.solves == "stream.top_k")
            .map(|algorithm| algorithm.id.as_str())
            .collect::<BTreeSet<_>>();
        let expected = registry
            .implementations
            .iter()
            .filter(|implementation| algorithms.contains(implementation.implements.as_str()))
            .map(|implementation| implementation.id.as_str())
            .collect::<BTreeSet<_>>();
        let actual = decisions
            .iter()
            .map(|decision| decision.candidate_id.as_str())
            .collect::<BTreeSet<_>>();

        assert_eq!(actual, expected);
        assert_eq!(decisions.len(), 2);
        assert!(decisions.iter().all(|decision| {
            !decision.accepted && decision.reasons == ["forbidden effect effect.allocates_storage"]
        }));
    }

    #[test]
    fn consumer_projection_discovers_a_new_conforming_manifest_without_code_changes() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut output = Vec::with_capacity(3);
        let capacity = output.capacity();
        caller_storage_top_k(&[4, 8, 1, 8, 7], 3, &mut output);
        assert_eq!(output, [8, 8, 7]);
        assert_eq!(output.capacity(), capacity);

        let contents = fs::read_to_string(workspace.join("registry/atlas.yaml")).unwrap();
        let mut document = serde_yaml::from_str::<serde_yaml::Value>(&contents).unwrap();
        let algorithms = document["algorithms"].as_sequence_mut().unwrap();
        let mut synthetic_algorithm = algorithms
            .iter()
            .find(|algorithm| algorithm["id"] == "stream.top_k.relaxed_selection")
            .unwrap()
            .clone();
        synthetic_algorithm["id"] = "test.stream.top_k.sorted_caller_buffer".into();
        synthetic_algorithm["name"]["value"] = "test-only sorted caller buffer".into();
        synthetic_algorithm["time_worst"]["value"] = "O(nk)".into();
        synthetic_algorithm["auxiliary_memory"]["value"] =
            "O(k) caller-provided output storage".into();
        for claim in ["name", "deterministic", "time_worst", "auxiliary_memory"] {
            synthetic_algorithm[claim]["source"] =
                "file:crates/atlas/src/decision_projection.rs".into();
        }
        algorithms.push(synthetic_algorithm);

        let implementations = document["implementations"].as_sequence_mut().unwrap();
        let mut synthetic = implementations
            .iter()
            .find(|implementation| implementation["implements"] == "stream.top_k.relaxed_selection")
            .unwrap()
            .clone();
        synthetic["id"] = "test.stream.top_k.caller_storage".into();
        synthetic["implements"] = "test.stream.top_k.sorted_caller_buffer".into();
        synthetic["version"]["value"] = "test fixture".into();
        synthetic["license"]["value"] = "MIT".into();
        synthetic["target"]["value"] = "Rust std test target".into();
        synthetic["dependencies"]["value"] = serde_yaml::Value::Sequence(Vec::new());
        synthetic["entrypoint"]["value"] =
            "decision_projection::tests::caller_storage_top_k".into();
        synthetic["signature"]["value"] =
            "fn caller_storage_top_k(values: &[i32], k: usize, output: &mut Vec<i32>)".into();
        synthetic["effects"]["value"]["allocation"] = "none".into();
        synthetic["tests"]["value"] = serde_yaml::Value::Sequence(vec![
            "decision_projection::tests::consumer_projection_discovers_a_new_conforming_manifest_without_code_changes".into(),
        ]);
        for claim in [
            "language",
            "version",
            "license",
            "target",
            "dependencies",
            "entrypoint",
            "signature",
            "effects",
            "tests",
        ] {
            synthetic[claim]["source"] = "file:crates/atlas/src/decision_projection.rs".into();
        }
        implementations.push(synthetic);

        let registry = serde_yaml::from_value::<Registry>(document).unwrap();
        assert!(registry.validate().is_empty());
        let overlay = project_exact_without_allocation(&registry, "stream.top_k").unwrap();
        let decisions = evaluate_request(&overlay, consumer_request_id()).unwrap();
        let added = decisions
            .iter()
            .find(|decision| decision.candidate_id == "test.stream.top_k.caller_storage")
            .expect("new manifest candidate must be projected");

        assert_eq!(decisions.len(), 3);
        assert!(added.accepted);
        assert!(added.reasons.is_empty());
    }

    #[test]
    fn consumer_projection_rejects_an_unknown_problem() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let registry = load_registry(&workspace.join("registry/atlas.yaml")).unwrap();

        assert_eq!(
            project_exact_without_allocation(&registry, "missing.problem").unwrap_err(),
            "unknown problem \"missing.problem\""
        );
    }

    #[test]
    fn conditioned_cost_projection_reports_public_schema_boundary() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let registry = load_registry(&workspace.join("registry/atlas.yaml")).unwrap();
        let overlay = project_conditioned_worst_time(&registry, "priority_queue.push").unwrap();
        let decisions = evaluate_request(&overlay, conditioned_cost_request_id()).unwrap();

        let algorithms = registry
            .algorithms
            .iter()
            .filter(|algorithm| algorithm.solves == "priority_queue.push")
            .map(|algorithm| algorithm.id.as_str())
            .collect::<BTreeSet<_>>();
        let expected = registry
            .implementations
            .iter()
            .filter(|implementation| algorithms.contains(implementation.implements.as_str()))
            .map(|implementation| implementation.id.as_str())
            .collect::<BTreeSet<_>>();
        let actual = decisions
            .iter()
            .map(|decision| decision.candidate_id.as_str())
            .collect::<BTreeSet<_>>();

        assert_eq!(actual, expected);
        assert_eq!(decisions.len(), 2);
        assert!(decisions.iter().all(|decision| {
            !decision.accepted
                && decision.reasons == ["missing exact cost profile push Time Worst O(log n)"]
        }));
    }
}
