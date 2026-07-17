use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::registry::Registry;

pub const SUPPORTED_OVERLAY_VERSION: &str = "phase2-km5-0";
const MAX_ATOMS: usize = 32;
const MAX_CANDIDATES: usize = 8;
const MAX_EQUIVALENCES: usize = 4;
const MAX_EQUIVALENCE_SIDE: usize = 3;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionOverlay {
    pub overlay_version: String,
    pub atoms: Vec<Atom>,
    pub candidates: Vec<Candidate>,
    pub relations: Vec<Relation>,
    #[serde(default)]
    pub(crate) equivalences: Vec<EncodingEquivalence>,
    pub requests: Vec<Request>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Atom {
    pub id: String,
    pub kind: AtomKind,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AtomKind {
    Capability,
    Condition,
    Guarantee,
    Effect,
    State,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    pub id: String,
    pub source: String,
    pub provides: Vec<Fact>,
    pub requires: Vec<Fact>,
    pub guarantees: Vec<Fact>,
    pub effects: Vec<Fact>,
    pub consumes_state: Option<Fact>,
    pub produces_state: Option<Fact>,
    pub costs: Vec<CostFact>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    pub atom: String,
    pub evidence: Evidence,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub level: OverlayEvidenceLevel,
    pub source: String,
    pub proof: Option<ProofMapping>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OverlayEvidenceLevel {
    Declared,
    Inferred,
    Tested,
    Observed,
    Proven,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofMapping {
    pub artifact: String,
    pub claim: String,
    pub review: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CostFact {
    pub operation: String,
    pub metric: CostMetric,
    pub regime: CostRegime,
    pub bound: String,
    pub requires: Vec<String>,
    pub evidence: Evidence,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CostMetric {
    Time,
    AuxiliaryMemory,
    RetainedMemory,
    Allocation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CostRegime {
    Worst,
    Expected,
    Amortized,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EncodingEquivalence {
    pub(crate) id: String,
    pub(crate) left: Vec<AssertionPattern>,
    pub(crate) right: Vec<AssertionPattern>,
    pub(crate) evidence: Evidence,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum AssertionPattern {
    Capability {
        atom: String,
    },
    Guarantee {
        atom: String,
    },
    Effect {
        atom: String,
    },
    Cost {
        operation: String,
        metric: CostMetric,
        regime: CostRegime,
        bound: String,
        requires: Vec<String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relation {
    pub from: String,
    pub to: String,
    pub kind: RelationKind,
    pub requires: Vec<String>,
    pub evidence: Evidence,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    ProjectsTo,
    Specializes,
    Finalizes,
    Refines,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub id: String,
    pub accepts: String,
    pub provides_conditions: Vec<String>,
    pub requires_guarantees: Vec<String>,
    pub forbids_effects: Vec<String>,
    pub consumes_state: Option<String>,
    pub maximum_costs: Vec<CostRequirement>,
    pub accepted_evidence: Vec<OverlayEvidenceLevel>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CostRequirement {
    pub operation: String,
    pub metric: CostMetric,
    pub regime: CostRegime,
    pub bound: String,
    pub requires: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct OverlayValidationError {
    pub field: String,
    pub message: String,
}

impl fmt::Display for OverlayValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.field, self.message)
    }
}

#[derive(Debug)]
pub enum OverlayLoadError {
    Read(std::io::Error),
    Parse(serde_yaml::Error),
    Invalid(Vec<OverlayValidationError>),
}

impl fmt::Display for OverlayLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(error) => write!(formatter, "cannot read decision overlay: {error}"),
            Self::Parse(error) => write!(formatter, "invalid decision-overlay YAML: {error}"),
            Self::Invalid(errors) => {
                writeln!(formatter, "decision overlay validation failed:")?;
                for error in errors {
                    writeln!(formatter, "  - {error}")?;
                }
                Ok(())
            }
        }
    }
}

pub fn load_decision_overlay(path: &Path) -> Result<DecisionOverlay, OverlayLoadError> {
    let contents = fs::read_to_string(path).map_err(OverlayLoadError::Read)?;
    parse_decision_overlay(&contents)
}

pub fn parse_decision_overlay(contents: &str) -> Result<DecisionOverlay, OverlayLoadError> {
    let overlay =
        serde_yaml::from_str::<DecisionOverlay>(contents).map_err(OverlayLoadError::Parse)?;
    let errors = overlay.validate();
    if errors.is_empty() {
        Ok(overlay)
    } else {
        Err(OverlayLoadError::Invalid(errors))
    }
}

pub fn validate_overlay_sources(
    overlay: &DecisionOverlay,
    registry: &Registry,
    workspace_root: &Path,
) -> Vec<OverlayValidationError> {
    let mut errors = Vec::new();
    for (index, candidate) in overlay.candidates.iter().enumerate() {
        let path = format!("candidates[{index}].source");
        if let Some(id) = candidate.source.strip_prefix("registry:") {
            let exists = registry.problems.iter().any(|entity| entity.id == id)
                || registry.algorithms.iter().any(|entity| entity.id == id)
                || registry
                    .implementations
                    .iter()
                    .any(|entity| entity.id == id);
            if !exists {
                errors.push(error(path, format!("unknown registry entity {id:?}")));
            }
        } else if let Some(relative) = candidate.source.strip_prefix("worksheet:") {
            let relative = Path::new(relative);
            if relative.is_absolute()
                || relative
                    .components()
                    .any(|component| matches!(component, std::path::Component::ParentDir))
            {
                errors.push(error(path, "worksheet path must be workspace-relative"));
            } else if !workspace_root.join(relative).is_file() {
                errors.push(error(
                    path,
                    format!("worksheet {:?} does not exist", relative.display()),
                ));
            }
        }
    }
    errors
}

impl DecisionOverlay {
    pub fn validate(&self) -> Vec<OverlayValidationError> {
        let mut errors = Vec::new();
        if self.overlay_version != SUPPORTED_OVERLAY_VERSION {
            errors.push(error(
                "overlay_version",
                format!(
                    "unsupported version {:?}; expected {:?}",
                    self.overlay_version, SUPPORTED_OVERLAY_VERSION
                ),
            ));
        }
        if self.atoms.len() > MAX_ATOMS {
            errors.push(error(
                "atoms",
                format!(
                    "contains {}; experiment limit is {MAX_ATOMS}",
                    self.atoms.len()
                ),
            ));
        }
        if self.candidates.len() > MAX_CANDIDATES {
            errors.push(error(
                "candidates",
                format!(
                    "contains {}; experiment limit is {MAX_CANDIDATES}",
                    self.candidates.len()
                ),
            ));
        }
        if self.equivalences.len() > MAX_EQUIVALENCES {
            errors.push(error(
                "equivalences",
                format!(
                    "contains {}; experiment limit is {MAX_EQUIVALENCES}",
                    self.equivalences.len()
                ),
            ));
        }

        let mut atom_kinds = HashMap::new();
        for (index, atom) in self.atoms.iter().enumerate() {
            validate_id(&format!("atoms[{index}].id"), &atom.id, &mut errors);
            if atom_kinds.insert(atom.id.as_str(), atom.kind).is_some() {
                errors.push(error(
                    format!("atoms[{index}].id"),
                    format!("duplicate atom ID {:?}", atom.id),
                ));
            }
        }

        let mut candidate_ids = HashSet::new();
        for (index, candidate) in self.candidates.iter().enumerate() {
            let path = format!("candidates[{index}]");
            validate_id(&format!("{path}.id"), &candidate.id, &mut errors);
            if !candidate_ids.insert(candidate.id.as_str()) {
                errors.push(error(
                    format!("{path}.id"),
                    format!("duplicate candidate ID {:?}", candidate.id),
                ));
            }
            if !candidate.source.starts_with("registry:")
                && !candidate.source.starts_with("worksheet:")
            {
                errors.push(error(
                    format!("{path}.source"),
                    "must start with registry: or worksheet:",
                ));
            }
            validate_facts(
                &format!("{path}.provides"),
                &candidate.provides,
                AtomKind::Capability,
                &atom_kinds,
                &mut errors,
            );
            validate_facts(
                &format!("{path}.requires"),
                &candidate.requires,
                AtomKind::Condition,
                &atom_kinds,
                &mut errors,
            );
            validate_facts(
                &format!("{path}.guarantees"),
                &candidate.guarantees,
                AtomKind::Guarantee,
                &atom_kinds,
                &mut errors,
            );
            validate_facts(
                &format!("{path}.effects"),
                &candidate.effects,
                AtomKind::Effect,
                &atom_kinds,
                &mut errors,
            );
            for (name, state) in [
                ("consumes_state", candidate.consumes_state.as_ref()),
                ("produces_state", candidate.produces_state.as_ref()),
            ] {
                if let Some(state) = state {
                    validate_fact(
                        &format!("{path}.{name}"),
                        state,
                        AtomKind::State,
                        &atom_kinds,
                        &mut errors,
                    );
                }
            }
            for (cost_index, cost) in candidate.costs.iter().enumerate() {
                let cost_path = format!("{path}.costs[{cost_index}]");
                validate_nonempty(
                    &format!("{cost_path}.operation"),
                    &cost.operation,
                    &mut errors,
                );
                validate_nonempty(&format!("{cost_path}.bound"), &cost.bound, &mut errors);
                validate_atom_refs(
                    &format!("{cost_path}.requires"),
                    &cost.requires,
                    AtomKind::Condition,
                    &atom_kinds,
                    &mut errors,
                );
                validate_evidence(
                    &format!("{cost_path}.evidence"),
                    &cost.evidence,
                    &mut errors,
                );
            }
        }

        for (index, relation) in self.relations.iter().enumerate() {
            let path = format!("relations[{index}]");
            let from_kind = if relation.kind == RelationKind::Finalizes {
                AtomKind::State
            } else {
                AtomKind::Capability
            };
            validate_atom_ref(
                &format!("{path}.from"),
                &relation.from,
                from_kind,
                &atom_kinds,
                &mut errors,
            );
            validate_atom_ref(
                &format!("{path}.to"),
                &relation.to,
                AtomKind::Capability,
                &atom_kinds,
                &mut errors,
            );
            validate_atom_refs(
                &format!("{path}.requires"),
                &relation.requires,
                AtomKind::Condition,
                &atom_kinds,
                &mut errors,
            );
            validate_evidence(&format!("{path}.evidence"), &relation.evidence, &mut errors);
        }

        let mut equivalence_ids = HashSet::new();
        for (index, equivalence) in self.equivalences.iter().enumerate() {
            let path = format!("equivalences[{index}]");
            validate_id(&format!("{path}.id"), &equivalence.id, &mut errors);
            if !equivalence_ids.insert(equivalence.id.as_str()) {
                errors.push(error(
                    format!("{path}.id"),
                    format!("duplicate equivalence ID {:?}", equivalence.id),
                ));
            }
            for (side_name, side) in [("left", &equivalence.left), ("right", &equivalence.right)] {
                if side.is_empty() || side.len() > MAX_EQUIVALENCE_SIDE {
                    errors.push(error(
                        format!("{path}.{side_name}"),
                        format!("must contain 1 to {MAX_EQUIVALENCE_SIDE} assertions"),
                    ));
                }
                let mut seen: Vec<&AssertionPattern> = Vec::new();
                for (assertion_index, assertion) in side.iter().enumerate() {
                    let assertion_path = format!("{path}.{side_name}[{assertion_index}]");
                    validate_assertion(&assertion_path, assertion, &atom_kinds, &mut errors);
                    if seen
                        .iter()
                        .any(|previous| assertion_patterns_match(previous, assertion))
                    {
                        errors.push(error(assertion_path, "duplicate assertion on this side"));
                    }
                    seen.push(assertion);
                }
            }
            if equivalence.left.iter().any(|left| {
                equivalence
                    .right
                    .iter()
                    .any(|right| assertion_patterns_match(left, right))
            }) {
                errors.push(error(path.clone(), "left and right sides must be disjoint"));
            }
            validate_evidence(
                &format!("{path}.evidence"),
                &equivalence.evidence,
                &mut errors,
            );
        }

        let mut request_ids = HashSet::new();
        for (index, request) in self.requests.iter().enumerate() {
            let path = format!("requests[{index}]");
            validate_id(&format!("{path}.id"), &request.id, &mut errors);
            if !request_ids.insert(request.id.as_str()) {
                errors.push(error(
                    format!("{path}.id"),
                    format!("duplicate request ID {:?}", request.id),
                ));
            }
            validate_atom_ref(
                &format!("{path}.accepts"),
                &request.accepts,
                AtomKind::Capability,
                &atom_kinds,
                &mut errors,
            );
            validate_atom_refs(
                &format!("{path}.provides_conditions"),
                &request.provides_conditions,
                AtomKind::Condition,
                &atom_kinds,
                &mut errors,
            );
            validate_atom_refs(
                &format!("{path}.requires_guarantees"),
                &request.requires_guarantees,
                AtomKind::Guarantee,
                &atom_kinds,
                &mut errors,
            );
            validate_atom_refs(
                &format!("{path}.forbids_effects"),
                &request.forbids_effects,
                AtomKind::Effect,
                &atom_kinds,
                &mut errors,
            );
            if let Some(state) = &request.consumes_state {
                validate_atom_ref(
                    &format!("{path}.consumes_state"),
                    state,
                    AtomKind::State,
                    &atom_kinds,
                    &mut errors,
                );
            }
            if request.accepted_evidence.is_empty() {
                errors.push(error(
                    format!("{path}.accepted_evidence"),
                    "must contain at least one exact accepted level",
                ));
            }
            let mut evidence_levels = HashSet::new();
            for (level_index, level) in request.accepted_evidence.iter().enumerate() {
                if !evidence_levels.insert(*level) {
                    errors.push(error(
                        format!("{path}.accepted_evidence[{level_index}]"),
                        format!("duplicate evidence level {level:?}"),
                    ));
                }
            }
            for (cost_index, cost) in request.maximum_costs.iter().enumerate() {
                let cost_path = format!("{path}.maximum_costs[{cost_index}]");
                validate_nonempty(
                    &format!("{cost_path}.operation"),
                    &cost.operation,
                    &mut errors,
                );
                validate_nonempty(&format!("{cost_path}.bound"), &cost.bound, &mut errors);
                validate_atom_refs(
                    &format!("{cost_path}.requires"),
                    &cost.requires,
                    AtomKind::Condition,
                    &atom_kinds,
                    &mut errors,
                );
            }
        }

        errors
    }
}

pub(crate) fn assertion_patterns_match(left: &AssertionPattern, right: &AssertionPattern) -> bool {
    match (left, right) {
        (
            AssertionPattern::Cost {
                operation: left_operation,
                metric: left_metric,
                regime: left_regime,
                bound: left_bound,
                requires: left_requires,
            },
            AssertionPattern::Cost {
                operation: right_operation,
                metric: right_metric,
                regime: right_regime,
                bound: right_bound,
                requires: right_requires,
            },
        ) => {
            left_operation == right_operation
                && left_metric == right_metric
                && left_regime == right_regime
                && left_bound == right_bound
                && left_requires.len() == right_requires.len()
                && left_requires
                    .iter()
                    .all(|item| right_requires.contains(item))
        }
        _ => left == right,
    }
}

fn validate_assertion(
    path: &str,
    assertion: &AssertionPattern,
    atoms: &HashMap<&str, AtomKind>,
    errors: &mut Vec<OverlayValidationError>,
) {
    match assertion {
        AssertionPattern::Capability { atom } => validate_atom_ref(
            &format!("{path}.atom"),
            atom,
            AtomKind::Capability,
            atoms,
            errors,
        ),
        AssertionPattern::Guarantee { atom } => validate_atom_ref(
            &format!("{path}.atom"),
            atom,
            AtomKind::Guarantee,
            atoms,
            errors,
        ),
        AssertionPattern::Effect { atom } => validate_atom_ref(
            &format!("{path}.atom"),
            atom,
            AtomKind::Effect,
            atoms,
            errors,
        ),
        AssertionPattern::Cost {
            operation,
            bound,
            requires,
            ..
        } => {
            validate_nonempty(&format!("{path}.operation"), operation, errors);
            validate_nonempty(&format!("{path}.bound"), bound, errors);
            validate_atom_refs(
                &format!("{path}.requires"),
                requires,
                AtomKind::Condition,
                atoms,
                errors,
            );
        }
    }
}

fn validate_facts(
    path: &str,
    facts: &[Fact],
    expected: AtomKind,
    atoms: &HashMap<&str, AtomKind>,
    errors: &mut Vec<OverlayValidationError>,
) {
    let mut seen = HashSet::new();
    for (index, fact) in facts.iter().enumerate() {
        validate_fact(&format!("{path}[{index}]"), fact, expected, atoms, errors);
        if !seen.insert(fact.atom.as_str()) {
            errors.push(error(
                format!("{path}[{index}].atom"),
                format!("duplicate fact for atom {:?}", fact.atom),
            ));
        }
    }
}

fn validate_fact(
    path: &str,
    fact: &Fact,
    expected: AtomKind,
    atoms: &HashMap<&str, AtomKind>,
    errors: &mut Vec<OverlayValidationError>,
) {
    validate_atom_ref(&format!("{path}.atom"), &fact.atom, expected, atoms, errors);
    validate_evidence(&format!("{path}.evidence"), &fact.evidence, errors);
}

fn validate_atom_refs(
    path: &str,
    values: &[String],
    expected: AtomKind,
    atoms: &HashMap<&str, AtomKind>,
    errors: &mut Vec<OverlayValidationError>,
) {
    let mut seen = HashSet::new();
    for (index, value) in values.iter().enumerate() {
        validate_atom_ref(&format!("{path}[{index}]"), value, expected, atoms, errors);
        if !seen.insert(value.as_str()) {
            errors.push(error(
                format!("{path}[{index}]"),
                format!("duplicate atom reference {value:?}"),
            ));
        }
    }
}

fn validate_atom_ref(
    path: &str,
    value: &str,
    expected: AtomKind,
    atoms: &HashMap<&str, AtomKind>,
    errors: &mut Vec<OverlayValidationError>,
) {
    match atoms.get(value) {
        Some(kind) if *kind == expected => {}
        Some(kind) => errors.push(error(
            path,
            format!("atom {value:?} has kind {kind:?}; expected {expected:?}"),
        )),
        None => errors.push(error(path, format!("unknown atom {value:?}"))),
    }
}

fn validate_evidence(path: &str, evidence: &Evidence, errors: &mut Vec<OverlayValidationError>) {
    validate_nonempty(&format!("{path}.source"), &evidence.source, errors);
    match (evidence.level, evidence.proof.as_ref()) {
        (OverlayEvidenceLevel::Proven, None) => errors.push(error(
            format!("{path}.proof"),
            "proven evidence requires artifact, claim, and review mapping",
        )),
        (OverlayEvidenceLevel::Proven, Some(proof)) => {
            validate_nonempty(&format!("{path}.proof.artifact"), &proof.artifact, errors);
            validate_nonempty(&format!("{path}.proof.claim"), &proof.claim, errors);
            validate_nonempty(&format!("{path}.proof.review"), &proof.review, errors);
        }
        (_, Some(_)) => errors.push(error(
            format!("{path}.proof"),
            "proof mapping is allowed only for proven evidence",
        )),
        (_, None) => {}
    }
}

fn validate_nonempty(path: &str, value: &str, errors: &mut Vec<OverlayValidationError>) {
    if value.trim().is_empty() {
        errors.push(error(path, "must not be empty"));
    }
}

fn validate_id(path: &str, value: &str, errors: &mut Vec<OverlayValidationError>) {
    if value.is_empty()
        || !value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
    {
        errors.push(error(
            path,
            "must contain only lowercase ASCII letters, digits, '.', '_' or '-'",
        ));
    }
}

fn error(field: impl Into<String>, message: impl Into<String>) -> OverlayValidationError {
    OverlayValidationError {
        field: field.into(),
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::registry::load_registry;

    use super::*;

    const VALID: &str = r#"
overlay_version: phase2-km5-0
atoms:
  - id: output.distances
    kind: capability
  - id: condition.nonnegative
    kind: condition
  - id: guarantee.exact
    kind: guarantee
  - id: effect.allocation
    kind: effect
  - id: state.graph
    kind: state
candidates:
  - id: candidate.dijkstra
    source: registry:graph.dijkstra.distances
    provides:
      - atom: output.distances
        evidence: { level: declared, source: "docs:fixture", proof: null }
    requires:
      - atom: condition.nonnegative
        evidence: { level: declared, source: "docs:fixture", proof: null }
    guarantees:
      - atom: guarantee.exact
        evidence: { level: tested, source: "tests:fixture", proof: null }
    effects: []
    consumes_state: null
    produces_state: null
    costs:
      - operation: run
        metric: time
        regime: worst
        bound: "O((V + E) log V)"
        requires: [condition.nonnegative]
        evidence: { level: declared, source: "docs:fixture", proof: null }
relations: []
requests:
  - id: request.distances
    accepts: output.distances
    provides_conditions: [condition.nonnegative]
    requires_guarantees: [guarantee.exact]
    forbids_effects: []
    consumes_state: null
    maximum_costs: []
    accepted_evidence: [declared, inferred, tested, observed, proven]
"#;

    #[test]
    fn accepts_the_bounded_shape() {
        let overlay = parse_decision_overlay(VALID).expect("valid overlay");
        assert_eq!(overlay.candidates.len(), 1);
        assert_eq!(overlay.requests.len(), 1);
    }

    #[test]
    fn rejects_unknown_fields_during_deserialization() {
        let invalid = VALID.replace("relations: []", "surprise: true\nrelations: []");
        let error = parse_decision_overlay(&invalid).expect_err("unknown field must fail");
        assert!(matches!(error, OverlayLoadError::Parse(_)));
        assert!(error.to_string().contains("unknown field `surprise`"));
    }

    #[test]
    fn rejects_dangling_and_wrong_kind_atom_references() {
        let invalid = VALID
            .replace(
                "accepts: output.distances",
                "accepts: condition.nonnegative",
            )
            .replace(
                "requires_guarantees: [guarantee.exact]",
                "requires_guarantees: [guarantee.missing]",
            );
        let error = parse_decision_overlay(&invalid).expect_err("invalid references must fail");
        let message = error.to_string();
        assert!(message.contains("has kind Condition; expected Capability"));
        assert!(message.contains("unknown atom \"guarantee.missing\""));
    }

    #[test]
    fn rejects_wrong_kind_and_tautological_equivalence_assertions() {
        let invalid = VALID.replace(
            "relations: []",
            r#"relations: []
equivalences:
  - id: equivalence.invalid
    left:
      - { kind: guarantee, atom: output.distances }
    right:
      - { kind: guarantee, atom: output.distances }
      - kind: cost
        operation: run
        metric: time
        regime: worst
        bound: O(n)
        requires: [condition.nonnegative]
    evidence: { level: declared, source: "docs:fixture", proof: null }"#,
        );
        let error = parse_decision_overlay(&invalid).expect_err("invalid mapping must fail");
        let message = error.to_string();
        assert!(message.contains("has kind Capability; expected Guarantee"));
        assert!(message.contains("left and right sides must be disjoint"));
    }

    #[test]
    fn rejects_proven_evidence_without_a_complete_mapping() {
        let invalid = VALID.replace(
            "level: tested, source: \"tests:fixture\", proof: null",
            "level: proven, source: \"docs:proof\", proof: null",
        );
        let error = parse_decision_overlay(&invalid).expect_err("unmapped proof must fail");
        assert!(
            error
                .to_string()
                .contains("proven evidence requires artifact, claim, and review mapping")
        );
    }

    #[test]
    fn accepts_proven_evidence_with_an_auditable_mapping() {
        let valid = VALID.replace(
            "level: tested, source: \"tests:fixture\", proof: null",
            "level: proven, source: \"docs:proof\", proof: { artifact: \"file:proof.md\", claim: \"theorem-1\", review: \"docs:review-1\" }",
        );
        parse_decision_overlay(&valid).expect("mapped proof must pass");
    }

    #[test]
    fn rejects_an_empty_or_duplicate_accepted_evidence_set() {
        for replacement in ["[]", "[declared, declared]"] {
            let invalid = VALID.replace(
                "[declared, inferred, tested, observed, proven]",
                replacement,
            );
            let error = parse_decision_overlay(&invalid)
                .expect_err("accepted evidence must be explicit and unique");
            assert!(error.to_string().contains("accepted_evidence"));
        }
    }

    #[test]
    fn rejects_duplicate_candidate_ids() {
        let mut overlay = parse_decision_overlay(VALID).expect("valid overlay");
        let duplicate = serde_yaml::from_str::<Candidate>(
            r#"
id: candidate.dijkstra
source: registry:graph.dijkstra.distances
provides: []
requires: []
guarantees: []
effects: []
consumes_state: null
produces_state: null
costs: []
"#,
        )
        .expect("candidate fixture");
        overlay.candidates.push(duplicate);
        assert!(
            overlay
                .validate()
                .iter()
                .any(|error| error.message.contains("duplicate candidate ID"))
        );
    }

    #[test]
    fn committed_overlay_sources_resolve_without_becoming_registry_authority() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let overlay = load_decision_overlay(&workspace.join("docs/phase2/k-m5-overlay.yaml"))
            .expect("committed overlay");
        let registry =
            load_registry(&workspace.join("registry/atlas.yaml")).expect("committed registry");
        assert!(validate_overlay_sources(&overlay, &registry, &workspace).is_empty());
    }
}
