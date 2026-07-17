use crate::decision_overlay::{
    Atom, AtomKind, Candidate, CostFact, CostMetric, CostRegime, CostRequirement, DecisionOverlay,
    Evidence, Fact, OverlayEvidenceLevel, Request, SUPPORTED_OVERLAY_VERSION,
};
use crate::registry::{Algorithm, Claim, EvidenceLevel, Implementation, Registry};

const CONTRACT: &str = "capability.problem_contract";
const ALLOCATES: &str = "effect.allocates_storage";
const REQUEST_ID: &str = "request.consumer.exact_without_allocation";
const SPARE_CAPACITY: &str = "condition.spare_capacity";
const NONADVERSARIAL_HASHING: &str = "condition.nonadversarial_hash_distribution";
const HEAP_COST_REQUEST_ID: &str = "request.consumer.log_push_with_spare_capacity";
const MAP_COST_REQUEST_ID: &str = "request.consumer.expected_map_insert";

struct ConditionedTimeRequest<'a> {
    id: &'a str,
    operation: &'a str,
    regime: CostRegime,
    bound: &'a str,
    condition: &'a str,
}

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

fn project_conditioned_time(
    registry: &Registry,
    problem_id: &str,
    request: ConditionedTimeRequest<'_>,
) -> Result<DecisionOverlay, String> {
    let candidates = candidates_for_problem(registry, problem_id)?
        .into_iter()
        .map(|(implementation, algorithm)| {
            let costs = time_claim(algorithm, request.regime)
                .map(|claim| CostFact {
                    operation: request.operation.into(),
                    metric: CostMetric::Time,
                    regime: request.regime,
                    bound: claim.value.clone(),
                    requires: Vec::new(),
                    evidence: evidence(claim.level, &claim.source),
                })
                .into_iter()
                .collect();
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
                effects: Vec::new(),
                consumes_state: None,
                produces_state: None,
                costs,
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
                id: request.condition.into(),
                kind: AtomKind::Condition,
            },
        ],
        candidates,
        relations: Vec::new(),
        equivalences: Vec::new(),
        requests: vec![Request {
            id: request.id.into(),
            accepts: CONTRACT.into(),
            provides_conditions: vec![request.condition.into()],
            requires_guarantees: Vec::new(),
            forbids_effects: Vec::new(),
            consumes_state: None,
            maximum_costs: vec![CostRequirement {
                operation: request.operation.into(),
                metric: CostMetric::Time,
                regime: request.regime,
                bound: request.bound.into(),
                requires: vec![request.condition.into()],
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

fn time_claim(algorithm: &Algorithm, regime: CostRegime) -> Option<&Claim<String>> {
    match regime {
        CostRegime::Worst => Some(&algorithm.time_worst),
        CostRegime::Expected => algorithm.time_expected.as_ref(),
        CostRegime::Amortized => None,
    }
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
        let overlay = project_conditioned_time(
            &registry,
            "priority_queue.push",
            ConditionedTimeRequest {
                id: HEAP_COST_REQUEST_ID,
                operation: "push",
                regime: CostRegime::Worst,
                bound: "O(log n)",
                condition: SPARE_CAPACITY,
            },
        )
        .unwrap();
        let decisions = evaluate_request(&overlay, HEAP_COST_REQUEST_ID).unwrap();

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

    #[test]
    fn conditioned_expected_cost_boundary_recurs_for_hash_map_insert() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let registry = load_registry(&workspace.join("registry/atlas.yaml")).unwrap();
        let overlay = project_conditioned_time(
            &registry,
            "associative_map.insert",
            ConditionedTimeRequest {
                id: MAP_COST_REQUEST_ID,
                operation: "insert",
                regime: CostRegime::Expected,
                bound: "O(1)",
                condition: NONADVERSARIAL_HASHING,
            },
        )
        .unwrap();
        let decisions = evaluate_request(&overlay, MAP_COST_REQUEST_ID).unwrap();

        let related_algorithm_ids = registry
            .algorithms
            .iter()
            .filter(|algorithm| algorithm.solves == "associative_map.insert")
            .map(|algorithm| algorithm.id.as_str())
            .collect::<BTreeSet<_>>();
        let expected_candidates = registry
            .implementations
            .iter()
            .filter(|implementation| {
                related_algorithm_ids.contains(implementation.implements.as_str())
            })
            .map(|implementation| implementation.id.as_str())
            .collect::<BTreeSet<_>>();
        let actual_candidates = decisions
            .iter()
            .map(|decision| decision.candidate_id.as_str())
            .collect::<BTreeSet<_>>();

        assert_eq!(actual_candidates, expected_candidates);
        assert_eq!(decisions.len(), 1);
        assert_eq!(
            decisions[0].reasons,
            ["missing exact cost profile insert Time Expected O(1)"]
        );
        assert!(!decisions[0].accepted);
    }

    #[test]
    fn generic_cost_fixture_covers_time_memory_and_allocation() {
        let spare = "condition.state.spare_capacity";
        let nonadversarial = "condition.workload.nonadversarial_hash_distribution";
        let heap = "capability.priority_queue.push";
        let map = "capability.associative_map.insert";
        let sort = "capability.sequence.sort";
        let cases = [
            FixtureCase::accepted(
                "request.heap.time.with_capacity",
                heap,
                Some(spare),
                CostMetric::Time,
                CostRegime::Worst,
                "O(log n)",
                Some(spare),
            ),
            FixtureCase::rejected(
                "request.heap.time.without_capacity",
                heap,
                CostMetric::Time,
                CostRegime::Worst,
                "O(log n)",
                Some(spare),
            ),
            FixtureCase::accepted(
                "request.heap.allocation.with_capacity",
                heap,
                Some(spare),
                CostMetric::Allocation,
                CostRegime::Worst,
                "none",
                Some(spare),
            ),
            FixtureCase::accepted(
                "request.map.time.nonadversarial",
                map,
                Some(nonadversarial),
                CostMetric::Time,
                CostRegime::Expected,
                "O(1)",
                Some(nonadversarial),
            ),
            FixtureCase::rejected(
                "request.map.time.unspecified_distribution",
                map,
                CostMetric::Time,
                CostRegime::Expected,
                "O(1)",
                Some(nonadversarial),
            ),
            FixtureCase::accepted(
                "request.sort.time",
                sort,
                None,
                CostMetric::Time,
                CostRegime::Worst,
                "O(n log n)",
                None,
            ),
            FixtureCase::accepted(
                "request.sort.memory",
                sort,
                None,
                CostMetric::AuxiliaryMemory,
                CostRegime::Worst,
                "O(n)",
                None,
            ),
        ];
        let overlay = DecisionOverlay {
            overlay_version: SUPPORTED_OVERLAY_VERSION.into(),
            atoms: [
                (heap, AtomKind::Capability),
                (map, AtomKind::Capability),
                (sort, AtomKind::Capability),
                (spare, AtomKind::Condition),
                (nonadversarial, AtomKind::Condition),
            ]
            .into_iter()
            .map(|(id, kind)| Atom {
                id: id.into(),
                kind,
            })
            .collect(),
            candidates: vec![
                fixture_candidate(
                    "fixture.heap",
                    heap,
                    vec![
                        fixture_cost(CostMetric::Time, CostRegime::Worst, "O(n)", &[]),
                        fixture_cost(CostMetric::Time, CostRegime::Worst, "O(log n)", &[spare]),
                        fixture_cost(CostMetric::Allocation, CostRegime::Worst, "none", &[spare]),
                    ],
                ),
                fixture_candidate(
                    "fixture.map",
                    map,
                    vec![fixture_cost(
                        CostMetric::Time,
                        CostRegime::Expected,
                        "O(1)",
                        &[nonadversarial],
                    )],
                ),
                fixture_candidate(
                    "fixture.sort",
                    sort,
                    vec![
                        fixture_cost(CostMetric::Time, CostRegime::Worst, "O(n log n)", &[]),
                        fixture_cost(CostMetric::AuxiliaryMemory, CostRegime::Worst, "O(n)", &[]),
                    ],
                ),
            ],
            relations: Vec::new(),
            equivalences: Vec::new(),
            requests: cases.iter().map(fixture_request).collect(),
        };

        let validation_errors = overlay.validate();
        assert!(
            validation_errors.is_empty(),
            "fixture validation failed: {validation_errors:#?}"
        );
        for case in &cases {
            let accepted = evaluate_request(&overlay, case.id)
                .unwrap()
                .into_iter()
                .filter(|decision| decision.accepted)
                .count();
            assert_eq!(accepted, usize::from(case.accepted), "case {}", case.id);
        }
        assert!(
            overlay
                .candidates
                .iter()
                .flat_map(|candidate| &candidate.costs)
                .all(|cost| cost.evidence.level == OverlayEvidenceLevel::Inferred
                    && cost.evidence.source == "fixture:phase8")
        );
    }

    fn fixture_candidate(id: &str, capability: &str, costs: Vec<CostFact>) -> Candidate {
        Candidate {
            id: id.into(),
            source: "worksheet:docs/phase8-qualified-time-design.md".into(),
            provides: vec![Fact {
                atom: capability.into(),
                evidence: fixture_evidence(),
            }],
            requires: Vec::new(),
            guarantees: Vec::new(),
            effects: Vec::new(),
            consumes_state: None,
            produces_state: None,
            costs,
        }
    }

    fn fixture_cost(
        metric: CostMetric,
        regime: CostRegime,
        bound: &str,
        requires: &[&str],
    ) -> CostFact {
        CostFact {
            operation: "solve".into(),
            metric,
            regime,
            bound: bound.into(),
            requires: requires
                .iter()
                .map(|condition| (*condition).into())
                .collect(),
            evidence: fixture_evidence(),
        }
    }

    struct FixtureCase<'a> {
        id: &'a str,
        capability: &'a str,
        condition: Option<&'a str>,
        metric: CostMetric,
        regime: CostRegime,
        bound: &'a str,
        requires: Option<&'a str>,
        accepted: bool,
    }

    impl<'a> FixtureCase<'a> {
        fn accepted(
            id: &'a str,
            capability: &'a str,
            condition: Option<&'a str>,
            metric: CostMetric,
            regime: CostRegime,
            bound: &'a str,
            requires: Option<&'a str>,
        ) -> Self {
            Self {
                id,
                capability,
                condition,
                metric,
                regime,
                bound,
                requires,
                accepted: true,
            }
        }

        fn rejected(
            id: &'a str,
            capability: &'a str,
            metric: CostMetric,
            regime: CostRegime,
            bound: &'a str,
            requires: Option<&'a str>,
        ) -> Self {
            Self {
                id,
                capability,
                condition: None,
                metric,
                regime,
                bound,
                requires,
                accepted: false,
            }
        }
    }

    fn fixture_request(case: &FixtureCase<'_>) -> Request {
        Request {
            id: case.id.into(),
            accepts: case.capability.into(),
            provides_conditions: case.condition.into_iter().map(String::from).collect(),
            requires_guarantees: Vec::new(),
            forbids_effects: Vec::new(),
            consumes_state: None,
            maximum_costs: vec![CostRequirement {
                operation: "solve".into(),
                metric: case.metric,
                regime: case.regime,
                bound: case.bound.into(),
                requires: case.requires.into_iter().map(String::from).collect(),
            }],
            accepted_evidence: vec![OverlayEvidenceLevel::Inferred],
        }
    }

    fn fixture_evidence() -> Evidence {
        Evidence {
            level: OverlayEvidenceLevel::Inferred,
            source: "fixture:phase8".into(),
            proof: None,
        }
    }
}
