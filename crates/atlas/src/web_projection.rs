use serde::Serialize;

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
    ) -> Self {
        Self {
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
        }
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
) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&WebProjection::new(registry, summary, source_commit))
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
        assert!(first.contains("order.is_sorted.adjacent"));
    }
}
