use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub const SUPPORTED_SCHEMA_VERSION: &str = "0.2";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    pub schema_version: String,
    pub conditions: Vec<Condition>,
    pub problems: Vec<Problem>,
    pub algorithms: Vec<Algorithm>,
    pub implementations: Vec<Implementation>,
    pub executions: Vec<serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Condition {
    pub id: String,
    pub statement: Claim<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Problem {
    pub id: String,
    pub input: Claim<String>,
    pub requires: Option<Claim<Vec<String>>>,
    pub output: Claim<String>,
    pub ensures: Claim<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Algorithm {
    pub id: String,
    pub solves: String,
    pub name: Claim<String>,
    pub requires: Option<Claim<Vec<String>>>,
    pub stable: Option<Claim<bool>>,
    pub deterministic: Claim<bool>,
    pub in_place: Option<Claim<bool>>,
    pub costs: Vec<Claim<CostProfile>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CostProfile {
    pub metric: CostMetric,
    pub regime: CostRegime,
    pub bound: String,
    pub requires: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CostMetric {
    Time,
    AuxiliaryMemory,
    RetainedMemory,
    Allocation,
}

impl fmt::Display for CostMetric {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Time => "time",
            Self::AuxiliaryMemory => "auxiliary_memory",
            Self::RetainedMemory => "retained_memory",
            Self::Allocation => "allocation",
        };
        formatter.write_str(value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CostRegime {
    Worst,
    Expected,
    Amortized,
}

impl fmt::Display for CostRegime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Worst => "worst",
            Self::Expected => "expected",
            Self::Amortized => "amortized",
        };
        formatter.write_str(value)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Implementation {
    pub id: String,
    pub implements: String,
    pub language: Claim<String>,
    pub version: Claim<String>,
    pub license: Claim<String>,
    pub target: Claim<String>,
    pub dependencies: Claim<Vec<String>>,
    pub abi: Claim<String>,
    pub entrypoint: Claim<String>,
    pub signature: Claim<String>,
    pub effects: Claim<Effects>,
    pub tests: Claim<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Effects {
    pub mutates: Vec<String>,
    pub io: String,
    pub blocking: bool,
    pub allocation: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Claim<T> {
    pub value: T,
    pub level: EvidenceLevel,
    pub source: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLevel {
    Declared,
    Inferred,
    Tested,
    Observed,
    Proven,
}

impl fmt::Display for EvidenceLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Declared => "declared",
            Self::Inferred => "inferred",
            Self::Tested => "tested",
            Self::Observed => "observed",
            Self::Proven => "proven",
        };
        formatter.write_str(value)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.field, self.message)
    }
}

#[derive(Debug)]
pub enum LoadError {
    Read(std::io::Error),
    Parse(serde_yaml::Error),
    Invalid(Vec<ValidationError>),
}

impl fmt::Display for LoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(error) => write!(formatter, "cannot read registry: {error}"),
            Self::Parse(error) => write!(formatter, "invalid YAML or schema shape: {error}"),
            Self::Invalid(errors) => {
                writeln!(formatter, "registry validation failed:")?;
                for error in errors {
                    writeln!(formatter, "  - {error}")?;
                }
                Ok(())
            }
        }
    }
}

pub fn load_registry(path: &Path) -> Result<Registry, LoadError> {
    let contents = fs::read_to_string(path).map_err(LoadError::Read)?;
    let registry: Registry = serde_yaml::from_str(&contents).map_err(LoadError::Parse)?;
    let mut errors = registry.validate();
    let workspace_root = workspace_root(path).unwrap_or_else(|| Path::new("."));
    errors.extend(registry.validate_evidence(workspace_root));
    if errors.is_empty() {
        Ok(registry)
    } else {
        Err(LoadError::Invalid(errors))
    }
}

impl Registry {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.schema_version != SUPPORTED_SCHEMA_VERSION {
            errors.push(error(
                "schema_version",
                format!(
                    "unsupported version {:?}; expected {:?}",
                    self.schema_version, SUPPORTED_SCHEMA_VERSION
                ),
            ));
        }

        let mut ids: HashMap<&str, &str> = HashMap::new();
        for (kind, id) in self
            .conditions
            .iter()
            .map(|item| ("condition", item.id.as_str()))
            .chain(
                self.problems
                    .iter()
                    .map(|item| ("problem", item.id.as_str())),
            )
            .chain(
                self.algorithms
                    .iter()
                    .map(|item| ("algorithm", item.id.as_str())),
            )
            .chain(
                self.implementations
                    .iter()
                    .map(|item| ("implementation", item.id.as_str())),
            )
        {
            validate_id(kind, id, &mut errors);
            if let Some(previous_kind) = ids.insert(id, kind) {
                errors.push(error(
                    format!("{kind}.{id}.id"),
                    format!("duplicates {previous_kind} ID {id:?}"),
                ));
            }
        }

        let problem_ids: Vec<_> = self.problems.iter().map(|item| item.id.as_str()).collect();
        let algorithm_ids: Vec<_> = self
            .algorithms
            .iter()
            .map(|item| item.id.as_str())
            .collect();
        let condition_ids: HashSet<_> = self
            .conditions
            .iter()
            .map(|condition| condition.id.as_str())
            .collect();

        for condition in &self.conditions {
            validate_claim(
                &format!("condition.{}.statement", condition.id),
                &condition.statement,
                &mut errors,
            );
            if condition.statement.value.trim().is_empty() {
                errors.push(error(
                    format!("condition.{}.statement.value", condition.id),
                    "must not be empty",
                ));
            }
        }

        for problem in &self.problems {
            validate_claim(
                &format!("problem.{}.input", problem.id),
                &problem.input,
                &mut errors,
            );
            validate_claim(
                &format!("problem.{}.output", problem.id),
                &problem.output,
                &mut errors,
            );
            validate_claim(
                &format!("problem.{}.ensures", problem.id),
                &problem.ensures,
                &mut errors,
            );
            if problem.ensures.value.is_empty() {
                errors.push(error(
                    format!("problem.{}.ensures.value", problem.id),
                    "must contain at least one postcondition",
                ));
            }
            if let Some(requires) = &problem.requires {
                validate_claim(
                    &format!("problem.{}.requires", problem.id),
                    requires,
                    &mut errors,
                );
                if requires.value.is_empty() {
                    errors.push(error(
                        format!("problem.{}.requires.value", problem.id),
                        "must contain at least one requirement when present",
                    ));
                }
            }
        }

        for algorithm in &self.algorithms {
            if !problem_ids.contains(&algorithm.solves.as_str()) {
                errors.push(error(
                    format!("algorithm.{}.solves", algorithm.id),
                    format!("references unknown problem {:?}", algorithm.solves),
                ));
            }
            for (name, claim) in [
                ("name", claim_ref(&algorithm.name)),
                ("deterministic", claim_ref(&algorithm.deterministic)),
            ] {
                validate_claim_parts(
                    &format!("algorithm.{}.{name}", algorithm.id),
                    claim,
                    &mut errors,
                );
            }
            if algorithm.costs.is_empty() {
                errors.push(error(
                    format!("algorithm.{}.costs", algorithm.id),
                    "must contain at least one cost profile",
                ));
            }
            let mut profiles = HashSet::new();
            for (index, cost) in algorithm.costs.iter().enumerate() {
                let path = format!("algorithm.{}.costs[{index}]", algorithm.id);
                validate_claim(&path, cost, &mut errors);
                if cost.value.bound.trim().is_empty() {
                    errors.push(error(format!("{path}.value.bound"), "must not be empty"));
                }
                let mut requirements = HashSet::new();
                for (requirement_index, requirement) in cost.value.requires.iter().enumerate() {
                    if !condition_ids.contains(requirement.as_str()) {
                        errors.push(error(
                            format!("{path}.value.requires[{requirement_index}]"),
                            format!("references unknown condition {requirement:?}"),
                        ));
                    }
                    if !requirements.insert(requirement.as_str()) {
                        errors.push(error(
                            format!("{path}.value.requires[{requirement_index}]"),
                            format!("duplicates condition {requirement:?}"),
                        ));
                    }
                }
                let identity = (
                    cost.value.metric,
                    cost.value.regime,
                    cost.value.bound.as_str(),
                    cost.value.requires.as_slice(),
                );
                if !profiles.insert(identity) {
                    errors.push(error(path, "duplicates an existing cost profile"));
                }
            }
            for (metric, regime) in [
                (CostMetric::Time, CostRegime::Worst),
                (CostMetric::AuxiliaryMemory, CostRegime::Worst),
            ] {
                if !algorithm
                    .costs
                    .iter()
                    .any(|cost| cost.value.metric == metric && cost.value.regime == regime)
                {
                    errors.push(error(
                        format!("algorithm.{}.costs", algorithm.id),
                        format!("must contain a {regime} {metric} profile"),
                    ));
                }
            }
            for (name, claim) in [
                ("stable", algorithm.stable.as_ref()),
                ("in_place", algorithm.in_place.as_ref()),
            ] {
                if let Some(claim) = claim {
                    validate_claim(
                        &format!("algorithm.{}.{name}", algorithm.id),
                        claim,
                        &mut errors,
                    );
                }
            }
            if let Some(requires) = &algorithm.requires {
                validate_claim(
                    &format!("algorithm.{}.requires", algorithm.id),
                    requires,
                    &mut errors,
                );
                if requires.value.is_empty() {
                    errors.push(error(
                        format!("algorithm.{}.requires.value", algorithm.id),
                        "must contain at least one requirement when present",
                    ));
                }
            }
        }

        for implementation in &self.implementations {
            if !algorithm_ids.contains(&implementation.implements.as_str()) {
                errors.push(error(
                    format!("implementation.{}.implements", implementation.id),
                    format!(
                        "references unknown algorithm {:?}",
                        implementation.implements
                    ),
                ));
            }
            for (name, claim) in [
                ("language", claim_ref(&implementation.language)),
                ("version", claim_ref(&implementation.version)),
                ("license", claim_ref(&implementation.license)),
                ("target", claim_ref(&implementation.target)),
                ("dependencies", claim_ref(&implementation.dependencies)),
                ("abi", claim_ref(&implementation.abi)),
                ("entrypoint", claim_ref(&implementation.entrypoint)),
                ("signature", claim_ref(&implementation.signature)),
                ("effects", claim_ref(&implementation.effects)),
                ("tests", claim_ref(&implementation.tests)),
            ] {
                validate_claim_parts(
                    &format!("implementation.{}.{name}", implementation.id),
                    claim,
                    &mut errors,
                );
            }
            if implementation.tests.value.is_empty() {
                errors.push(error(
                    format!("implementation.{}.tests.value", implementation.id),
                    "must contain at least one test identifier",
                ));
            }
        }

        if !self.executions.is_empty() {
            errors.push(error(
                "executions",
                "execution records are not defined by schema 0.2 yet; expected an empty list",
            ));
        }

        errors
    }

    pub fn validate_evidence(&self, workspace_root: &Path) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let implementation_ids: Vec<_> = self
            .implementations
            .iter()
            .map(|implementation| implementation.id.as_str())
            .collect();

        let mut check = |path: String, source: &str| {
            validate_evidence_source(
                &path,
                source,
                workspace_root,
                &implementation_ids,
                &mut errors,
            );
        };

        for condition in &self.conditions {
            check(
                format!("condition.{}.statement.source", condition.id),
                &condition.statement.source,
            );
        }

        for problem in &self.problems {
            check(
                format!("problem.{}.input.source", problem.id),
                &problem.input.source,
            );
            if let Some(requires) = &problem.requires {
                check(
                    format!("problem.{}.requires.source", problem.id),
                    &requires.source,
                );
            }
            check(
                format!("problem.{}.output.source", problem.id),
                &problem.output.source,
            );
            check(
                format!("problem.{}.ensures.source", problem.id),
                &problem.ensures.source,
            );
        }
        for algorithm in &self.algorithms {
            for (name, source) in [
                ("name", algorithm.name.source.as_str()),
                ("deterministic", algorithm.deterministic.source.as_str()),
            ] {
                check(format!("algorithm.{}.{name}.source", algorithm.id), source);
            }
            for (name, claim) in [
                ("requires", algorithm.requires.as_ref().map(claim_ref)),
                ("stable", algorithm.stable.as_ref().map(claim_ref)),
                ("in_place", algorithm.in_place.as_ref().map(claim_ref)),
            ] {
                if let Some((_level, source)) = claim {
                    check(format!("algorithm.{}.{name}.source", algorithm.id), source);
                }
            }
            for (index, cost) in algorithm.costs.iter().enumerate() {
                check(
                    format!("algorithm.{}.costs[{index}].source", algorithm.id),
                    &cost.source,
                );
            }
        }
        for implementation in &self.implementations {
            for (name, source) in [
                ("language", implementation.language.source.as_str()),
                ("version", implementation.version.source.as_str()),
                ("license", implementation.license.source.as_str()),
                ("target", implementation.target.source.as_str()),
                ("dependencies", implementation.dependencies.source.as_str()),
                ("abi", implementation.abi.source.as_str()),
                ("entrypoint", implementation.entrypoint.source.as_str()),
                ("signature", implementation.signature.source.as_str()),
                ("effects", implementation.effects.source.as_str()),
                ("tests", implementation.tests.source.as_str()),
            ] {
                check(
                    format!("implementation.{}.{name}.source", implementation.id),
                    source,
                );
            }
        }

        errors
    }
}

fn workspace_root(registry_path: &Path) -> Option<&Path> {
    registry_path
        .parent()
        .into_iter()
        .flat_map(Path::ancestors)
        .find(|ancestor| ancestor.join("Cargo.toml").is_file())
}

fn validate_evidence_source(
    path: &str,
    source: &str,
    workspace_root: &Path,
    implementation_ids: &[&str],
    errors: &mut Vec<ValidationError>,
) {
    let Some((scheme, targets)) = source.split_once(':') else {
        errors.push(error(
            path,
            "provenance must have a recognized scheme followed by ':'",
        ));
        return;
    };
    if targets.is_empty() || targets.split(';').any(str::is_empty) {
        errors.push(error(path, "provenance targets must not be empty"));
        return;
    }

    match scheme {
        "file" => validate_file_source(path, targets, workspace_root, errors),
        "implementation" | "implementations" => {
            for target in targets.split(';') {
                if !implementation_ids.contains(&target) {
                    errors.push(error(
                        path,
                        format!("references unknown implementation {target:?}"),
                    ));
                }
            }
        }
        "test" | "tests" => {
            for target in targets.split(';') {
                validate_test_source(path, target, workspace_root, errors);
            }
        }
        "analysis" | "command" | "definition" | "docs" | "vision" => {}
        _ => errors.push(error(
            path,
            format!("uses unsupported provenance scheme {scheme:?}"),
        )),
    }
}

fn validate_file_source(
    path: &str,
    target: &str,
    workspace_root: &Path,
    errors: &mut Vec<ValidationError>,
) {
    let relative = Path::new(target);
    if relative.is_absolute()
        || relative
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        errors.push(error(
            path,
            "file provenance must be a workspace-relative path",
        ));
    } else if !workspace_root.join(relative).is_file() {
        errors.push(error(
            path,
            format!("references missing workspace file {target:?}"),
        ));
    }
}

fn validate_test_source(
    path: &str,
    target: &str,
    workspace_root: &Path,
    errors: &mut Vec<ValidationError>,
) {
    let Some((module, test_name)) = target.split_once("::") else {
        errors.push(error(
            path,
            format!("invalid Rust test reference {target:?}"),
        ));
        return;
    };
    let valid_identifier = |value: &str| {
        !value.is_empty()
            && value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    };
    if !valid_identifier(module) || !valid_identifier(test_name) || test_name.contains("::") {
        errors.push(error(
            path,
            format!("invalid Rust test reference {target:?}"),
        ));
        return;
    }

    let source_path = workspace_root
        .join("crates/atlas-algorithms/src")
        .join(format!("{module}.rs"));
    let Ok(contents) = fs::read_to_string(&source_path) else {
        errors.push(error(
            path,
            format!("test module file does not exist for {target:?}"),
        ));
        return;
    };
    let declaration = format!("fn {test_name}(");
    let mut test_attribute_seen = false;
    let found = contents.lines().any(|line| {
        let line = line.trim();
        if line.is_empty() {
            return false;
        }
        if line == "#[test]" {
            test_attribute_seen = true;
            return false;
        }
        if test_attribute_seen && line.starts_with("#[") {
            return false;
        }
        let matches = test_attribute_seen && line.starts_with(&declaration);
        test_attribute_seen = false;
        matches
    });
    if !found {
        errors.push(error(path, format!("Rust test {target:?} was not found")));
    }
}

fn validate_id(kind: &str, id: &str, errors: &mut Vec<ValidationError>) {
    let valid = !id.is_empty()
        && id.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        });
    if !valid {
        errors.push(error(
            format!("{kind}.{id}.id"),
            "must use lowercase ASCII letters, digits, dots, underscores, or hyphens",
        ));
    }
}

fn validate_claim<T>(path: &str, claim: &Claim<T>, errors: &mut Vec<ValidationError>) {
    validate_claim_parts(path, claim_ref(claim), errors);
}

fn claim_ref<T>(claim: &Claim<T>) -> (&EvidenceLevel, &str) {
    (&claim.level, claim.source.as_str())
}

fn validate_claim_parts(
    path: &str,
    (_level, source): (&EvidenceLevel, &str),
    errors: &mut Vec<ValidationError>,
) {
    if source.trim().is_empty() {
        errors.push(error(
            format!("{path}.source"),
            "provenance source must not be empty",
        ));
    }
}

fn error(field: impl Into<String>, message: impl Into<String>) -> ValidationError {
    ValidationError {
        field: field.into(),
        message: message.into(),
    }
}
