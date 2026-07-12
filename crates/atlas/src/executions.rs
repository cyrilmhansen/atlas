use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const EXPERIMENTAL_EXECUTION_FORMAT: &str = "atlas-execution.experimental.0.2";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionBody {
    pub format: String,
    pub recipe_id: String,
    pub mode: ExecutionMode,
    pub implementation_id: String,
    pub dataset: ExecutionDataset,
    pub parameters: ExecutionParameters,
    pub environment: ExecutionEnvironment,
    pub result: CorrectionResult,
    pub provenance: ExecutionProvenance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionRecord {
    pub id: String,
    #[serde(flatten)]
    pub body: ExecutionBody,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMode {
    Correction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionDataset {
    pub spec_id: String,
    pub case_id: String,
    pub content_digest_sha256: String,
    pub seed: u64,
    pub element_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionParameters {
    pub value_type: String,
    pub operation: String,
    pub build_profile: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionEnvironment {
    pub git_commit: String,
    pub git_dirty: bool,
    pub compiler: String,
    pub target: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CorrectionResult {
    pub passed: bool,
    pub outputs: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionProvenance {
    pub command: String,
    pub recipe_source: String,
    pub implementation_source: String,
}

#[derive(Debug)]
pub struct ExecutionError(String);

impl fmt::Display for ExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ExecutionError {}

impl ExecutionRecord {
    pub fn from_body(body: ExecutionBody) -> Result<Self, ExecutionError> {
        let id = body_id(&body)?;
        Ok(Self { id, body })
    }

    pub fn validate_id(&self) -> Result<(), ExecutionError> {
        let expected = body_id(&self.body)?;
        if self.id == expected {
            Ok(())
        } else {
            Err(ExecutionError(format!(
                "execution ID mismatch: expected {expected:?}, found {:?}",
                self.id
            )))
        }
    }

    pub fn validate(&self) -> Result<(), ExecutionError> {
        if self.body.format != EXPERIMENTAL_EXECUTION_FORMAT {
            return Err(ExecutionError(format!(
                "unsupported execution format {:?}; expected {:?}",
                self.body.format, EXPERIMENTAL_EXECUTION_FORMAT
            )));
        }
        self.validate_id()
    }

    pub fn from_yaml(input: &str) -> Result<Self, ExecutionError> {
        let record: Self = serde_yaml::from_str(input)
            .map_err(|error| ExecutionError(format!("cannot parse execution: {error}")))?;
        record.validate()?;
        Ok(record)
    }

    pub fn to_yaml(&self) -> Result<String, ExecutionError> {
        serde_yaml::to_string(self)
            .map_err(|error| ExecutionError(format!("cannot serialize execution: {error}")))
    }

    pub fn write_yaml(&self, path: &Path) -> Result<(), ExecutionError> {
        self.validate()?;
        let parent = path.parent().ok_or_else(|| {
            ExecutionError(format!("execution path {:?} has no parent directory", path))
        })?;
        fs::create_dir_all(parent).map_err(|error| {
            ExecutionError(format!(
                "cannot create execution directory {:?}: {error}",
                parent
            ))
        })?;
        fs::write(path, self.to_yaml()?)
            .map_err(|error| ExecutionError(format!("cannot write execution {:?}: {error}", path)))
    }
}

pub fn digest_i32_values(values: &[i32]) -> String {
    let mut hash = Sha256::new();
    hash.update(values.len().to_le_bytes());
    for value in values {
        hash.update(value.to_le_bytes());
    }
    hex_digest(hash.finalize().as_slice())
}

fn body_id(body: &ExecutionBody) -> Result<String, ExecutionError> {
    let encoded = serde_yaml::to_string(body)
        .map_err(|error| ExecutionError(format!("cannot identify execution: {error}")))?;
    let digest = Sha256::digest(encoded.as_bytes());
    Ok(format!("execution.sha256.{}", hex_digest(&digest)))
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use fmt::Write;
        write!(&mut encoded, "{byte:02x}").expect("writing to a String cannot fail");
    }
    encoded
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        CorrectionResult, EXPERIMENTAL_EXECUTION_FORMAT, ExecutionBody, ExecutionDataset,
        ExecutionEnvironment, ExecutionMode, ExecutionParameters, ExecutionProvenance,
        ExecutionRecord,
    };

    fn body() -> ExecutionBody {
        ExecutionBody {
            format: EXPERIMENTAL_EXECUTION_FORMAT.to_owned(),
            recipe_id: "sort.insertion.uniform.64.correction.v1".to_owned(),
            mode: ExecutionMode::Correction,
            implementation_id: "sort.insertion.rust.slice.v1".to_owned(),
            dataset: ExecutionDataset {
                spec_id: "dataset.sequence.sort.benchmark.m2.v0".to_owned(),
                case_id: "sort.benchmark.uniform.64".to_owned(),
                content_digest_sha256: "a".repeat(64),
                seed: 64,
                element_count: 64,
            },
            parameters: ExecutionParameters {
                value_type: "i32".to_owned(),
                operation: "i32::cmp".to_owned(),
                build_profile: "debug".to_owned(),
            },
            environment: ExecutionEnvironment {
                git_commit: "0123456789abcdef".to_owned(),
                git_dirty: false,
                compiler: "rustc test".to_owned(),
                target: "test-target".to_owned(),
            },
            result: CorrectionResult {
                passed: true,
                outputs: BTreeMap::from([("sequence_digest_sha256".to_owned(), "b".repeat(64))]),
            },
            provenance: ExecutionProvenance {
                command: "cargo run -p atlas --example record_sort_correction".to_owned(),
                recipe_source: "file:crates/atlas/examples/record_sort_correction.rs".to_owned(),
                implementation_source: "file:crates/atlas-algorithms/src/insertion_sort.rs"
                    .to_owned(),
            },
        }
    }

    #[test]
    fn serialization_and_identity_are_deterministic() {
        let first = ExecutionRecord::from_body(body()).unwrap();
        let second = ExecutionRecord::from_body(body()).unwrap();

        assert_eq!(first, second);
        let yaml = first.to_yaml().unwrap();
        assert_eq!(yaml, second.to_yaml().unwrap());
        assert_eq!(ExecutionRecord::from_yaml(&yaml).unwrap(), first);
        assert!(first.id.starts_with("execution.sha256."));
        first.validate().unwrap();
    }

    #[test]
    fn identity_validation_detects_modified_observations() {
        let mut record = ExecutionRecord::from_body(body()).unwrap();
        record.body.result.passed = false;

        let error = record.validate_id().unwrap_err();
        assert!(error.to_string().contains("execution ID mismatch"));
    }
}
