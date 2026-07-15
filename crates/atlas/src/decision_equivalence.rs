use std::collections::HashSet;

use crate::decision_overlay::{
    AssertionPattern, Candidate, CostFact, DecisionOverlay, OverlayEvidenceLevel,
    assertion_patterns_match,
};

pub(crate) fn supports(
    overlay: &DecisionOverlay,
    candidate: &Candidate,
    target: &AssertionPattern,
    accepted: &HashSet<OverlayEvidenceLevel>,
    conditions: &HashSet<&str>,
) -> bool {
    supports_with_evidence(overlay, candidate, target, Some(accepted), conditions)
}

pub(crate) fn supports_without_evidence(
    overlay: &DecisionOverlay,
    candidate: &Candidate,
    target: &AssertionPattern,
    conditions: &HashSet<&str>,
) -> bool {
    supports_with_evidence(overlay, candidate, target, None, conditions)
}

fn supports_with_evidence(
    overlay: &DecisionOverlay,
    candidate: &Candidate,
    target: &AssertionPattern,
    accepted: Option<&HashSet<OverlayEvidenceLevel>>,
    conditions: &HashSet<&str>,
) -> bool {
    assertion_conditions_met(target, conditions)
        && (direct_evidence(candidate, target, conditions)
            .is_some_and(|level| allows(level, accepted))
            || overlay.equivalences.iter().any(|equivalence| {
                allows(equivalence.evidence.level, accepted)
                    && (derives(
                        candidate,
                        target,
                        &equivalence.left,
                        &equivalence.right,
                        accepted,
                        conditions,
                    ) || derives(
                        candidate,
                        target,
                        &equivalence.right,
                        &equivalence.left,
                        accepted,
                        conditions,
                    ))
            }))
}

fn derives(
    candidate: &Candidate,
    target: &AssertionPattern,
    target_side: &[AssertionPattern],
    source_side: &[AssertionPattern],
    accepted: Option<&HashSet<OverlayEvidenceLevel>>,
    conditions: &HashSet<&str>,
) -> bool {
    target_side
        .iter()
        .any(|assertion| assertion_patterns_match(assertion, target))
        && source_side.iter().all(|assertion| {
            direct_evidence(candidate, assertion, conditions)
                .is_some_and(|level| allows(level, accepted))
        })
}

fn direct_evidence(
    candidate: &Candidate,
    assertion: &AssertionPattern,
    conditions: &HashSet<&str>,
) -> Option<OverlayEvidenceLevel> {
    if !assertion_conditions_met(assertion, conditions) {
        return None;
    }
    match assertion {
        AssertionPattern::Capability { atom } => candidate
            .provides
            .iter()
            .find(|fact| fact.atom == *atom)
            .map(|fact| fact.evidence.level),
        AssertionPattern::Guarantee { atom } => candidate
            .guarantees
            .iter()
            .find(|fact| fact.atom == *atom)
            .map(|fact| fact.evidence.level),
        AssertionPattern::Effect { atom } => candidate
            .effects
            .iter()
            .find(|fact| fact.atom == *atom)
            .map(|fact| fact.evidence.level),
        AssertionPattern::Cost { .. } => candidate
            .costs
            .iter()
            .find(|cost| cost_matches_pattern(cost, assertion))
            .map(|cost| cost.evidence.level),
    }
}

fn assertion_conditions_met(assertion: &AssertionPattern, conditions: &HashSet<&str>) -> bool {
    match assertion {
        AssertionPattern::Cost { requires, .. } => requires
            .iter()
            .all(|condition| conditions.contains(condition.as_str())),
        _ => true,
    }
}

fn cost_matches_pattern(cost: &CostFact, pattern: &AssertionPattern) -> bool {
    let AssertionPattern::Cost {
        operation,
        metric,
        regime,
        bound,
        requires,
    } = pattern
    else {
        return false;
    };
    cost.operation == *operation
        && cost.metric == *metric
        && cost.regime == *regime
        && cost.bound == *bound
        && cost.requires.len() == requires.len()
        && cost.requires.iter().all(|item| requires.contains(item))
}

fn allows(level: OverlayEvidenceLevel, accepted: Option<&HashSet<OverlayEvidenceLevel>>) -> bool {
    accepted.is_none_or(|levels| levels.contains(&level))
}
