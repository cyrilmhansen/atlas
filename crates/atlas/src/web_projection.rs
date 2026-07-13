use std::fmt;

use serde::Serialize;

use crate::datasets::{DatasetClass, GenerationError, SORT_DATASET_SPEC};
use crate::index::ProjectionSummary;
use crate::registry::{Algorithm, Claim, Implementation, Problem, Registry};

pub const WEB_PROJECTION_FORMAT: &str = "atlas-web-private-v0";

#[derive(Serialize)]
pub struct WebProjection<'a> {
    format: &'static str,
    source_commit: &'a str,
    registry_digest: &'a str,
    counts: WebCounts,
    problems: Vec<WebProblem<'a>>,
    algorithms: Vec<WebAlgorithm<'a>>,
    implementations: Vec<WebImplementation<'a>>,
    datasets: Vec<WebDataset>,
}

#[derive(Serialize)]
struct WebCounts {
    problems: usize,
    algorithms: usize,
    implementations: usize,
}

#[derive(Serialize)]
struct WebProblem<'a> {
    id: &'a str,
    input: WebClaim<'a, String>,
    output: WebClaim<'a, String>,
    ensures: WebClaim<'a, Vec<String>>,
}

#[derive(Serialize)]
struct WebAlgorithm<'a> {
    id: &'a str,
    solves: &'a str,
    name: WebClaim<'a, String>,
    time_worst: WebClaim<'a, String>,
    auxiliary_memory: WebClaim<'a, String>,
}

#[derive(Serialize)]
struct WebImplementation<'a> {
    id: &'a str,
    implements: &'a str,
    language: WebClaim<'a, String>,
    target: WebClaim<'a, String>,
    entrypoint: WebClaim<'a, String>,
}

#[derive(Serialize)]
struct WebDataset {
    spec_id: &'static str,
    case_id: &'static str,
    problem_id: &'static str,
    class: &'static str,
    seed: u64,
    values: Vec<i32>,
    content_digest_sha256: String,
}

#[derive(Serialize)]
struct WebClaim<'a, T> {
    value: &'a T,
    level: String,
    source: &'a str,
}

impl<'a> WebProjection<'a> {
    pub fn new(
        registry: &'a Registry,
        summary: &'a ProjectionSummary,
        source_commit: &'a str,
    ) -> Result<Self, GenerationError> {
        let datasets = SORT_DATASET_SPEC
            .generate_all()?
            .into_iter()
            .map(|dataset| WebDataset {
                spec_id: dataset.spec_id,
                case_id: dataset.case_id,
                problem_id: dataset.problem_id,
                class: dataset_class_name(dataset.class),
                seed: dataset.seed,
                values: dataset.values,
                content_digest_sha256: dataset.content_digest_sha256,
            })
            .collect();

        Ok(Self {
            format: WEB_PROJECTION_FORMAT,
            source_commit,
            registry_digest: &summary.digest,
            counts: WebCounts {
                problems: registry.problems.len(),
                algorithms: registry.algorithms.len(),
                implementations: registry.implementations.len(),
            },
            problems: registry.problems.iter().map(WebProblem::from).collect(),
            algorithms: registry.algorithms.iter().map(WebAlgorithm::from).collect(),
            implementations: registry
                .implementations
                .iter()
                .map(WebImplementation::from)
                .collect(),
            datasets,
        })
    }
}

impl<'a, T> From<&'a Claim<T>> for WebClaim<'a, T> {
    fn from(claim: &'a Claim<T>) -> Self {
        Self {
            value: &claim.value,
            level: claim.level.to_string(),
            source: &claim.source,
        }
    }
}

impl<'a> From<&'a Problem> for WebProblem<'a> {
    fn from(problem: &'a Problem) -> Self {
        Self {
            id: &problem.id,
            input: (&problem.input).into(),
            output: (&problem.output).into(),
            ensures: (&problem.ensures).into(),
        }
    }
}

impl<'a> From<&'a Algorithm> for WebAlgorithm<'a> {
    fn from(algorithm: &'a Algorithm) -> Self {
        Self {
            id: &algorithm.id,
            solves: &algorithm.solves,
            name: (&algorithm.name).into(),
            time_worst: (&algorithm.time_worst).into(),
            auxiliary_memory: (&algorithm.auxiliary_memory).into(),
        }
    }
}

impl<'a> From<&'a Implementation> for WebImplementation<'a> {
    fn from(implementation: &'a Implementation) -> Self {
        Self {
            id: &implementation.id,
            implements: &implementation.implements,
            language: (&implementation.language).into(),
            target: (&implementation.target).into(),
            entrypoint: (&implementation.entrypoint).into(),
        }
    }
}

pub fn to_json(
    registry: &Registry,
    summary: &ProjectionSummary,
    source_commit: &str,
) -> Result<String, WebProjectionError> {
    let projection = WebProjection::new(registry, summary, source_commit)?;
    Ok(serde_json::to_string_pretty(&projection)?)
}

fn dataset_class_name(class: DatasetClass) -> &'static str {
    match class {
        DatasetClass::Typical => "typical",
        DatasetClass::Boundary => "boundary",
        DatasetClass::Degenerate => "degenerate",
        DatasetClass::Adversarial => "adversarial",
        DatasetClass::Regression => "regression",
    }
}

#[derive(Debug)]
pub enum WebProjectionError {
    Dataset(GenerationError),
    Json(serde_json::Error),
}

impl fmt::Display for WebProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dataset(error) => write!(formatter, "cannot generate Web dataset: {error}"),
            Self::Json(error) => write!(formatter, "cannot serialize Web projection: {error}"),
        }
    }
}

impl std::error::Error for WebProjectionError {}

impl From<GenerationError> for WebProjectionError {
    fn from(error: GenerationError) -> Self {
        Self::Dataset(error)
    }
}

impl From<serde_json::Error> for WebProjectionError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[cfg(test)]
mod tests {
    use crate::index::summarize_registry;
    use crate::registry::Registry;

    use super::{WEB_PROJECTION_FORMAT, to_json};

    const REGISTRY: &str = include_str!("../../../registry/atlas.yaml");

    #[test]
    fn projection_is_deterministic_and_carries_authority_identity() {
        let registry: Registry = serde_yaml::from_str(REGISTRY).unwrap();
        let summary = summarize_registry(&registry).unwrap();

        let first = to_json(&registry, &summary, "0123456789abcdef").unwrap();
        let second = to_json(&registry, &summary, "0123456789abcdef").unwrap();

        assert_eq!(first, second);
        let value: serde_json::Value = serde_json::from_str(&first).unwrap();
        assert_eq!(value["format"], WEB_PROJECTION_FORMAT);
        assert_eq!(value["source_commit"], "0123456789abcdef");
        assert_eq!(value["registry_digest"], summary.digest);
        assert_eq!(value["counts"]["problems"], 10);
        assert_eq!(value["counts"]["algorithms"], 15);
        assert_eq!(value["counts"]["implementations"], 20);
        assert_eq!(value["datasets"].as_array().unwrap().len(), 5);
        assert_eq!(
            value["datasets"][4]["case_id"],
            "sort.regression.duplicates"
        );
        assert_eq!(
            value["datasets"][4]["values"],
            serde_json::json!([5, -1, 5, 3, 0, -8, 3])
        );
        assert_eq!(value["datasets"][4]["class"], "regression");
        assert_eq!(
            value["datasets"][4]["content_digest_sha256"]
                .as_str()
                .unwrap()
                .len(),
            64
        );
        assert!(first.contains("order.is_sorted.adjacent"));
    }
}
