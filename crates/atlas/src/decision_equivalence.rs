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
) -> bool {
    supports_with_evidence(overlay, candidate, target, Some(accepted))
}

pub(crate) fn supports_without_evidence(
    overlay: &DecisionOverlay,
    candidate: &Candidate,
    target: &AssertionPattern,
) -> bool {
    supports_with_evidence(overlay, candidate, target, None)
}

fn supports_with_evidence(
    overlay: &DecisionOverlay,
    candidate: &Candidate,
    target: &AssertionPattern,
    accepted: Option<&HashSet<OverlayEvidenceLevel>>,
) -> bool {
    direct_evidence(candidate, target).is_some_and(|level| allows(level, accepted))
        || overlay.equivalences.iter().any(|equivalence| {
            allows(equivalence.evidence.level, accepted)
                && (derives(
                    candidate,
                    target,
                    &equivalence.left,
                    &equivalence.right,
                    accepted,
                ) || derives(
                    candidate,
                    target,
                    &equivalence.right,
                    &equivalence.left,
                    accepted,
                ))
        })
}

fn derives(
    candidate: &Candidate,
    target: &AssertionPattern,
    target_side: &[AssertionPattern],
    source_side: &[AssertionPattern],
    accepted: Option<&HashSet<OverlayEvidenceLevel>>,
) -> bool {
    target_side
        .iter()
        .any(|assertion| assertion_patterns_match(assertion, target))
        && source_side.iter().all(|assertion| {
            direct_evidence(candidate, assertion).is_some_and(|level| allows(level, accepted))
        })
}

fn direct_evidence(
    candidate: &Candidate,
    assertion: &AssertionPattern,
) -> Option<OverlayEvidenceLevel> {
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
