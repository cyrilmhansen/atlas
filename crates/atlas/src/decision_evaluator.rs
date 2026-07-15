use std::collections::HashSet;

use crate::decision_overlay::{
    Candidate, CostFact, CostRequirement, DecisionOverlay, Fact, Relation, Request,
};

#[derive(Debug, Eq, PartialEq)]
pub struct CandidateDecision {
    pub candidate_id: String,
    pub accepted: bool,
    pub reasons: Vec<String>,
}

pub fn evaluate_request(
    overlay: &DecisionOverlay,
    request_id: &str,
) -> Result<Vec<CandidateDecision>, String> {
    let request = overlay
        .requests
        .iter()
        .find(|request| request.id == request_id)
        .ok_or_else(|| format!("unknown decision-overlay request {request_id:?}"))?;

    Ok(overlay
        .candidates
        .iter()
        .map(|candidate| evaluate_candidate(overlay, candidate, request))
        .collect())
}

fn evaluate_candidate(
    overlay: &DecisionOverlay,
    candidate: &Candidate,
    request: &Request,
) -> CandidateDecision {
    let conditions = request
        .provides_conditions
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let accepted_evidence = request
        .accepted_evidence
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    let mut reasons = Vec::new();

    if let Err(mut capability_reasons) =
        capability_matches(overlay, candidate, request, &conditions, &accepted_evidence)
    {
        reasons.append(&mut capability_reasons);
    }

    for requirement in &candidate.requires {
        if !conditions.contains(requirement.atom.as_str()) {
            reasons.push(format!("missing condition {}", requirement.atom));
        } else if !accepted_evidence.contains(&requirement.evidence.level) {
            reasons.push(format!(
                "condition {} has unaccepted evidence {:?}",
                requirement.atom, requirement.evidence.level
            ));
        }
    }

    for required in &request.requires_guarantees {
        match candidate
            .guarantees
            .iter()
            .find(|fact| fact.atom == *required)
        {
            Some(fact) if accepted_evidence.contains(&fact.evidence.level) => {}
            Some(fact) => reasons.push(format!(
                "guarantee {required} has unaccepted evidence {:?}",
                fact.evidence.level
            )),
            None => reasons.push(format!("missing guarantee {required}")),
        }
    }

    for forbidden in &request.forbids_effects {
        if candidate.effects.iter().any(|fact| fact.atom == *forbidden) {
            reasons.push(format!("forbidden effect {forbidden}"));
        }
    }

    if let Some(required_state) = &request.consumes_state {
        match &candidate.consumes_state {
            Some(fact) if fact.atom == *required_state => {
                if !accepted_evidence.contains(&fact.evidence.level) {
                    reasons.push(format!(
                        "state {required_state} has unaccepted evidence {:?}",
                        fact.evidence.level
                    ));
                }
            }
            Some(fact) => reasons.push(format!(
                "state mismatch: requires {required_state}, candidate consumes {}",
                fact.atom
            )),
            None => reasons.push(format!("missing consumed state {required_state}")),
        }
    }

    for required_cost in &request.maximum_costs {
        match candidate
            .costs
            .iter()
            .find(|cost| cost_matches(cost, required_cost))
        {
            Some(cost) if accepted_evidence.contains(&cost.evidence.level) => {
                for condition in &cost.requires {
                    if !conditions.contains(condition.as_str()) {
                        reasons.push(format!(
                            "cost {} {} requires condition {condition}",
                            cost.operation, cost.bound
                        ));
                    }
                }
            }
            Some(cost) => reasons.push(format!(
                "cost {} {} has unaccepted evidence {:?}",
                cost.operation, cost.bound, cost.evidence.level
            )),
            None => reasons.push(format!(
                "missing exact cost profile {} {:?} {:?} {}",
                required_cost.operation,
                required_cost.metric,
                required_cost.regime,
                required_cost.bound
            )),
        }
    }

    reasons.sort();
    reasons.dedup();
    CandidateDecision {
        candidate_id: candidate.id.clone(),
        accepted: reasons.is_empty(),
        reasons,
    }
}

fn capability_matches(
    overlay: &DecisionOverlay,
    candidate: &Candidate,
    request: &Request,
    conditions: &HashSet<&str>,
    accepted_evidence: &HashSet<crate::decision_overlay::OverlayEvidenceLevel>,
) -> Result<(), Vec<String>> {
    let mut path_failures = Vec::new();
    for fact in candidate
        .provides
        .iter()
        .chain(candidate.produces_state.iter())
    {
        if fact.atom == request.accepts {
            if accepted_evidence.contains(&fact.evidence.level) {
                return Ok(());
            }
            path_failures.push(unaccepted_capability_reason(fact));
        }
        for relation in overlay
            .relations
            .iter()
            .filter(|relation| relation.from == fact.atom && relation.to == request.accepts)
        {
            match relation_path_failure(fact, relation, conditions, accepted_evidence) {
                None => return Ok(()),
                Some(reason) => path_failures.push(reason),
            }
        }
    }
    if path_failures.is_empty() {
        path_failures.push(format!("does not provide capability {}", request.accepts));
    }
    Err(path_failures)
}

fn relation_path_failure(
    fact: &Fact,
    relation: &Relation,
    conditions: &HashSet<&str>,
    accepted_evidence: &HashSet<crate::decision_overlay::OverlayEvidenceLevel>,
) -> Option<String> {
    if !accepted_evidence.contains(&fact.evidence.level) {
        return Some(unaccepted_capability_reason(fact));
    }
    if !accepted_evidence.contains(&relation.evidence.level) {
        return Some(format!(
            "relation {} -> {} has unaccepted evidence {:?}",
            relation.from, relation.to, relation.evidence.level
        ));
    }
    let missing = relation
        .requires
        .iter()
        .filter(|required| !conditions.contains(required.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    if missing.is_empty() {
        None
    } else {
        Some(format!(
            "relation {} -> {} requires conditions {}",
            relation.from,
            relation.to,
            missing.join(", ")
        ))
    }
}

fn unaccepted_capability_reason(fact: &Fact) -> String {
    format!(
        "capability {} has unaccepted evidence {:?}",
        fact.atom, fact.evidence.level
    )
}

fn cost_matches(cost: &CostFact, required: &CostRequirement) -> bool {
    cost.operation == required.operation
        && cost.metric == required.metric
        && cost.regime == required.regime
        && cost.bound == required.bound
        && cost.requires.len() == required.requires.len()
        && cost
            .requires
            .iter()
            .all(|condition| required.requires.contains(condition))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::decision_overlay::{load_decision_overlay, validate_overlay_sources};
    use crate::registry::load_registry;

    fn fixture() -> DecisionOverlay {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        load_decision_overlay(&workspace.join("docs/phase2/k-m5-overlay.yaml"))
            .expect("committed K-M5 overlay must validate")
    }

    #[test]
    fn independent_top_k_submission_matches_the_frozen_oracle() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let overlay = load_decision_overlay(
            &workspace.join("docs/phase2/imports/k-m5/independent-topk/submission.yaml"),
        )
        .expect("independent submission must validate without repair");
        let registry =
            load_registry(&workspace.join("registry/atlas.yaml")).expect("committed registry");
        assert!(validate_overlay_sources(&overlay, &registry, &workspace).is_empty());

        let cases = [
            ("request.empty_when_capacity_zero", true),
            ("request.exact_with_bounded_retention", true),
            ("request.exact_without_allocation", false),
        ];
        for (request_id, expected) in cases {
            let decision = evaluate_request(&overlay, request_id)
                .expect("mapped request")
                .into_iter()
                .find(|decision| decision.candidate_id == "candidate.bounded_top_k.binary_heap")
                .expect("independently authored candidate");
            assert_eq!(decision.accepted, expected, "request {request_id}");
            if request_id == "request.exact_without_allocation" {
                assert_eq!(
                    decision.reasons,
                    ["forbidden effect effect.allocates.retained_and_output_storage"]
                );
            }
        }
    }

    #[test]
    fn all_adjudicated_requests_match_their_expected_candidate_sets() {
        let overlay = fixture();
        let cases = [
            (
                "request.dijkstra.distances",
                &["candidate.dijkstra.path_tree"][..],
            ),
            (
                "request.heavy_hitters.exact",
                &["candidate.misra_gries.verified"][..],
            ),
            (
                "request.heap_push.no_allocation",
                &["candidate.heap_push"][..],
            ),
            ("request.heap_push.individual_log", &[][..]),
            (
                "request.disjoint_union.state",
                &["candidate.disjoint_union"][..],
            ),
            ("request.bloom.exact", &[][..]),
            (
                "request.bloom.definitive_negative",
                &["candidate.bloom_query"][..],
            ),
            ("request.moments.unbiased_small", &[][..]),
            ("request.moments.bitwise_pairwise", &[][..]),
            ("request.moments.proven_pairwise", &[][..]),
        ];

        for (request_id, expected) in cases {
            let accepted = evaluate_request(&overlay, request_id)
                .expect("known request")
                .into_iter()
                .filter(|decision| decision.accepted)
                .map(|decision| decision.candidate_id)
                .collect::<Vec<_>>();
            assert_eq!(accepted, expected, "request {request_id}");
        }
    }

    #[test]
    fn negative_candidates_explain_the_decision_relevant_fact() {
        let overlay = fixture();
        let cases = [
            (
                "request.heavy_hitters.exact",
                "candidate.misra_gries.candidates",
                "does not provide capability",
            ),
            (
                "request.heap_push.individual_log",
                "candidate.heap_push",
                "missing exact cost profile",
            ),
            (
                "request.bloom.exact",
                "candidate.bloom_query",
                "missing guarantee guarantee.exact",
            ),
            (
                "request.moments.unbiased_small",
                "candidate.moments.incremental",
                "requires conditions condition.count_gt_1",
            ),
            (
                "request.moments.bitwise_pairwise",
                "candidate.moments.pairwise",
                "missing guarantee guarantee.bitwise_order_independent",
            ),
            (
                "request.moments.proven_pairwise",
                "candidate.moments.pairwise",
                "unaccepted evidence Declared",
            ),
        ];

        for (request_id, candidate_id, expected_reason) in cases {
            let decision = evaluate_request(&overlay, request_id)
                .expect("known request")
                .into_iter()
                .find(|decision| decision.candidate_id == candidate_id)
                .expect("known candidate");
            assert!(!decision.accepted);
            assert!(
                decision
                    .reasons
                    .iter()
                    .any(|reason| reason.contains(expected_reason)),
                "{request_id}/{candidate_id}: {:?}",
                decision.reasons
            );
        }
    }

    #[test]
    fn decisions_are_data_driven() {
        let mut overlay = fixture();
        let exact = overlay
            .candidates
            .iter()
            .find(|candidate| candidate.id == "candidate.misra_gries.verified")
            .and_then(|candidate| candidate.guarantees.first())
            .expect("exact guarantee")
            .clone();
        let candidate = overlay
            .candidates
            .iter_mut()
            .find(|candidate| candidate.id == "candidate.bloom_query")
            .expect("Bloom candidate");
        candidate.guarantees.push(exact);

        let decision = evaluate_request(&overlay, "request.bloom.exact")
            .expect("known request")
            .into_iter()
            .find(|decision| decision.candidate_id == "candidate.bloom_query")
            .expect("Bloom candidate");
        assert!(decision.accepted);
    }

    #[test]
    fn unknown_request_is_actionable() {
        let error = evaluate_request(&fixture(), "request.missing").expect_err("must reject");
        assert!(error.contains("unknown decision-overlay request"));
    }

    #[test]
    fn cost_conditions_are_compared_as_sets() {
        let fact = serde_yaml::from_str::<CostFact>(
            r#"
operation: push
metric: time
regime: worst
bound: O(log n)
requires: [condition.a, condition.b]
evidence: { level: declared, source: "docs:fixture", proof: null }
"#,
        )
        .expect("cost fact");
        let requirement = serde_yaml::from_str::<CostRequirement>(
            r#"
operation: push
metric: time
regime: worst
bound: O(log n)
requires: [condition.b, condition.a]
"#,
        )
        .expect("cost requirement");
        assert!(cost_matches(&fact, &requirement));
    }
}
