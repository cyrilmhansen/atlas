use std::fmt;

use serde::Serialize;

use crate::ast::{minimum_ast, partition_ast, reverse_ast};
use crate::datasets::{
    DatasetClass, GenerationError, IntPredicate, PARTITION_DATASET_SPEC, SORT_DATASET_SPEC,
};
use crate::index::ProjectionSummary;
use crate::registry::{
    Algorithm, Claim, Condition, CostProfile, Effects, Implementation, Problem, Registry,
};
use crate::visual_program::{
    VisualProgram, VisualProgramError, compile_insertion_visual_program,
    compile_is_sorted_visual_program, compile_minimum_visual_program,
    compile_partition_even_visual_program, compile_reverse_visual_program,
};

pub const WEB_PROJECTION_FORMAT: &str = "atlas-web-private-v1";

#[derive(Serialize)]
pub struct WebProjection<'a> {
    format: &'static str,
    source_commit: &'a str,
    registry_digest: &'a str,
    build: WebBuildEnvironment<'a>,
    counts: WebCounts,
    conditions: Vec<WebCondition<'a>>,
    problems: Vec<WebProblem<'a>>,
    algorithms: Vec<WebAlgorithm<'a>>,
    implementations: Vec<WebImplementation<'a>>,
    datasets: Vec<WebDataset>,
    dynamics: Vec<WebDynamics>,
}

#[derive(Clone, Copy, Serialize)]
pub struct WebBuildEnvironment<'a> {
    pub rustc: &'a str,
    pub wasm_bindgen: &'a str,
    pub target: &'static str,
    pub profile: &'static str,
}

#[derive(Serialize)]
struct WebCounts {
    conditions: usize,
    problems: usize,
    algorithms: usize,
    implementations: usize,
}

#[derive(Serialize)]
struct WebCondition<'a> {
    id: &'a str,
    statement: WebClaim<'a, String>,
}

#[derive(Serialize)]
struct WebProblem<'a> {
    id: &'a str,
    input: WebClaim<'a, String>,
    requires: Option<WebClaim<'a, Vec<String>>>,
    output: WebClaim<'a, String>,
    ensures: WebClaim<'a, Vec<String>>,
}

#[derive(Serialize)]
struct WebAlgorithm<'a> {
    id: &'a str,
    solves: &'a str,
    name: WebClaim<'a, String>,
    requires: Option<WebClaim<'a, Vec<String>>>,
    stable: Option<WebClaim<'a, bool>>,
    deterministic: WebClaim<'a, bool>,
    in_place: Option<WebClaim<'a, bool>>,
    costs: Vec<WebClaim<'a, CostProfile>>,
}

#[derive(Serialize)]
struct WebImplementation<'a> {
    id: &'a str,
    implements: &'a str,
    language: WebClaim<'a, String>,
    version: WebClaim<'a, String>,
    license: WebClaim<'a, String>,
    target: WebClaim<'a, String>,
    dependencies: WebClaim<'a, Vec<String>>,
    abi: WebClaim<'a, String>,
    entrypoint: WebClaim<'a, String>,
    signature: WebClaim<'a, String>,
    effects: WebEffectsClaim<'a>,
    tests: WebClaim<'a, Vec<String>>,
}

#[derive(Serialize)]
struct WebEffectsClaim<'a> {
    value: WebEffects<'a>,
    level: String,
    source: &'a str,
}

#[derive(Serialize)]
struct WebEffects<'a> {
    mutates: &'a Vec<String>,
    io: &'a str,
    blocking: bool,
    allocation: &'a str,
}

#[derive(Serialize)]
struct WebDataset {
    spec_id: &'static str,
    case_id: &'static str,
    problem_id: &'static str,
    class: &'static str,
    seed: u64,
    values: Vec<i32>,
    predicate: Option<String>,
    content_digest_sha256: String,
}

#[derive(Serialize)]
struct WebDynamics {
    algorithm_id: &'static str,
    ast_id: &'static str,
    pseudocode_source: &'static str,
    max_interactive_input_length: usize,
    max_analytical_trace_input_length: usize,
    program: Option<VisualProgram>,
    presentation: Option<WebPresentation>,
}

#[derive(Serialize)]
struct WebPresentation {
    key: &'static str,
    selector_label: &'static str,
    primitive: &'static str,
    default_dataset: &'static str,
    dataset_problem_id: &'static str,
    dataset_predicate: Option<&'static str>,
    boundary: &'static str,
    result_label: &'static str,
    primary_counter_label: &'static str,
    secondary_label: &'static str,
    sequence_heading: &'static str,
    legend: &'static str,
    comparison_interest: &'static str,
    result_view: &'static str,
    primary_counter: &'static str,
    secondary_counter: &'static str,
    highlight: &'static str,
    tracks_origins: bool,
    predicate_label: Option<&'static str>,
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
        build: WebBuildEnvironment<'a>,
    ) -> Result<Self, WebProjectionError> {
        let datasets = SORT_DATASET_SPEC
            .generate_all()?
            .into_iter()
            .chain(PARTITION_DATASET_SPEC.generate_all()?)
            .map(|dataset| WebDataset {
                spec_id: dataset.spec_id,
                case_id: dataset.case_id,
                problem_id: dataset.problem_id,
                class: dataset_class_name(dataset.class),
                seed: dataset.seed,
                values: dataset.values,
                predicate: dataset.predicate.map(predicate_name),
                content_digest_sha256: dataset.content_digest_sha256,
            })
            .collect();

        Ok(Self {
            format: WEB_PROJECTION_FORMAT,
            source_commit,
            registry_digest: &summary.digest,
            build,
            counts: WebCounts {
                conditions: registry.conditions.len(),
                problems: registry.problems.len(),
                algorithms: registry.algorithms.len(),
                implementations: registry.implementations.len(),
            },
            conditions: registry.conditions.iter().map(WebCondition::from).collect(),
            problems: registry.problems.iter().map(WebProblem::from).collect(),
            algorithms: registry.algorithms.iter().map(WebAlgorithm::from).collect(),
            implementations: registry
                .implementations
                .iter()
                .map(WebImplementation::from)
                .collect(),
            datasets,
            dynamics: vec![
                WebDynamics {
                    algorithm_id: "order.is_sorted.adjacent",
                    ast_id: "ast.order.is_sorted.adjacent.v0",
                    pseudocode_source: include_str!("../pseudocode/is_sorted.atlas-pseudo"),
                    max_interactive_input_length: 64,
                    max_analytical_trace_input_length: 64,
                    program: Some(compile_is_sorted_visual_program(
                        &crate::ast::is_sorted_ast(),
                    )?),
                    presentation: Some(WebPresentation {
                        key: "is_sorted",
                        selector_label: "Is sorted",
                        primitive: "sequence",
                        default_dataset: "sort.degenerate.equal",
                        dataset_problem_id: "sequence.sort",
                        dataset_predicate: None,
                        boundary: "The generated read-only program and current sequence stay in WASM; only current values and counters are copied for display.",
                        result_label: "Result",
                        primary_counter_label: "Comparisons",
                        secondary_label: "First inversion",
                        sequence_heading: "Sequence state",
                        legend: "first decreasing pair",
                        comparison_interest: "greater",
                        result_view: "sortedness",
                        primary_counter: "comparisons",
                        secondary_counter: "result_index",
                        highlight: "first_inversion",
                        tracks_origins: false,
                        predicate_label: None,
                    }),
                },
                WebDynamics {
                    algorithm_id: "sort.insertion",
                    ast_id: "ast.sort.insertion.v0",
                    pseudocode_source: include_str!("../pseudocode/insertion_sort.atlas-pseudo"),
                    max_interactive_input_length: 64,
                    max_analytical_trace_input_length: 32,
                    program: Some(compile_insertion_visual_program(
                        &crate::ast::insertion_sort_ast(),
                    )?),
                    presentation: Some(WebPresentation {
                        key: "insertion",
                        selector_label: "Insertion sort",
                        primitive: "sequence",
                        default_dataset: "sort.regression.duplicates",
                        dataset_problem_id: "sequence.sort",
                        dataset_predicate: None,
                        boundary: "The generated stable insertion program mutates values and lazily tracks origins in WASM; current state is copied for display.",
                        result_label: "Correction + stability",
                        primary_counter_label: "Comparisons",
                        secondary_label: "Adjacent swaps",
                        sequence_heading: "Stable sorted output",
                        legend: "moved from original index",
                        comparison_interest: "less",
                        result_view: "stable_sorted",
                        primary_counter: "comparisons",
                        secondary_counter: "swaps",
                        highlight: "original_indices",
                        tracks_origins: true,
                        predicate_label: None,
                    }),
                },
                WebDynamics {
                    algorithm_id: "reverse.symmetric.in_place",
                    ast_id: "ast.reverse.symmetric.in_place.v0",
                    pseudocode_source: include_str!("../pseudocode/reverse.atlas-pseudo"),
                    max_interactive_input_length: 64,
                    max_analytical_trace_input_length: 0,
                    program: Some(compile_reverse_visual_program(&reverse_ast())?),
                    presentation: Some(WebPresentation {
                        key: "reverse",
                        selector_label: "Reverse",
                        primitive: "sequence",
                        default_dataset: "sort.regression.duplicates",
                        dataset_problem_id: "sequence.sort",
                        dataset_predicate: None,
                        boundary: "The generated symmetric reverse program mutates values and lazily tracks origins in WASM; current state is copied for display.",
                        result_label: "Correction + involution",
                        primary_counter_label: "Semantic reads / writes",
                        secondary_label: "Symmetric swaps",
                        sequence_heading: "Reversed output",
                        legend: "moved from original index",
                        comparison_interest: "none",
                        result_view: "reversed",
                        primary_counter: "reads",
                        secondary_counter: "swaps",
                        highlight: "original_indices",
                        tracks_origins: true,
                        predicate_label: None,
                    }),
                },
                WebDynamics {
                    algorithm_id: "select.minimum.linear",
                    ast_id: "ast.select.minimum.linear.v0",
                    pseudocode_source: include_str!("../pseudocode/minimum.atlas-pseudo"),
                    max_interactive_input_length: 64,
                    max_analytical_trace_input_length: 0,
                    program: Some(compile_minimum_visual_program(&minimum_ast())?),
                    presentation: Some(WebPresentation {
                        key: "minimum",
                        selector_label: "Minimum",
                        primitive: "sequence",
                        default_dataset: "sort.regression.duplicates",
                        dataset_problem_id: "sequence.sort",
                        dataset_predicate: None,
                        boundary: "The generated program and current sequence stay in WASM; the selected result is copied for display.",
                        result_label: "First minimum",
                        primary_counter_label: "Comparisons",
                        secondary_label: "Minimum index",
                        sequence_heading: "Minimum selection",
                        legend: "first minimum value",
                        comparison_interest: "less",
                        result_view: "optional_value",
                        primary_counter: "comparisons",
                        secondary_counter: "result_index",
                        highlight: "selected_index",
                        tracks_origins: false,
                        predicate_label: None,
                    }),
                },
                WebDynamics {
                    algorithm_id: "partition.two_pointer.in_place",
                    ast_id: "ast.partition.two_pointer.in_place.v0",
                    pseudocode_source: include_str!("../pseudocode/partition.atlas-pseudo"),
                    max_interactive_input_length: 64,
                    max_analytical_trace_input_length: 0,
                    program: Some(compile_partition_even_visual_program(&partition_ast())?),
                    presentation: Some(WebPresentation {
                        key: "partition",
                        selector_label: "Partition",
                        primitive: "sequence",
                        default_dataset: "partition.adversarial.alternating",
                        dataset_problem_id: "sequence.partition",
                        dataset_predicate: Some("even"),
                        boundary: "The generated even-predicate program mutates the sequence in WASM; current values and origins are copied for display.",
                        result_label: "Boundary",
                        primary_counter_label: "Predicate evaluations",
                        secondary_label: "Swaps",
                        sequence_heading: "Even / odd in-place partition",
                        legend: "values matching is_even",
                        comparison_interest: "none",
                        result_view: "partition_boundary",
                        primary_counter: "predicate_evaluations",
                        secondary_counter: "swaps",
                        highlight: "partition_boundary",
                        tracks_origins: true,
                        predicate_label: Some("is_even"),
                    }),
                },
            ],
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
            requires: problem.requires.as_ref().map(Into::into),
            output: (&problem.output).into(),
            ensures: (&problem.ensures).into(),
        }
    }
}

impl<'a> From<&'a Condition> for WebCondition<'a> {
    fn from(condition: &'a Condition) -> Self {
        Self {
            id: &condition.id,
            statement: (&condition.statement).into(),
        }
    }
}

impl<'a> From<&'a Algorithm> for WebAlgorithm<'a> {
    fn from(algorithm: &'a Algorithm) -> Self {
        Self {
            id: &algorithm.id,
            solves: &algorithm.solves,
            name: (&algorithm.name).into(),
            requires: algorithm.requires.as_ref().map(Into::into),
            stable: algorithm.stable.as_ref().map(Into::into),
            deterministic: (&algorithm.deterministic).into(),
            in_place: algorithm.in_place.as_ref().map(Into::into),
            costs: algorithm.costs.iter().map(Into::into).collect(),
        }
    }
}

impl<'a> From<&'a Implementation> for WebImplementation<'a> {
    fn from(implementation: &'a Implementation) -> Self {
        Self {
            id: &implementation.id,
            implements: &implementation.implements,
            language: (&implementation.language).into(),
            version: (&implementation.version).into(),
            license: (&implementation.license).into(),
            target: (&implementation.target).into(),
            dependencies: (&implementation.dependencies).into(),
            abi: (&implementation.abi).into(),
            entrypoint: (&implementation.entrypoint).into(),
            signature: (&implementation.signature).into(),
            effects: (&implementation.effects).into(),
            tests: (&implementation.tests).into(),
        }
    }
}

impl<'a> From<&'a Claim<Effects>> for WebEffectsClaim<'a> {
    fn from(claim: &'a Claim<Effects>) -> Self {
        Self {
            value: WebEffects {
                mutates: &claim.value.mutates,
                io: &claim.value.io,
                blocking: claim.value.blocking,
                allocation: &claim.value.allocation,
            },
            level: claim.level.to_string(),
            source: &claim.source,
        }
    }
}

pub fn to_json(
    registry: &Registry,
    summary: &ProjectionSummary,
    source_commit: &str,
    build: WebBuildEnvironment<'_>,
) -> Result<String, WebProjectionError> {
    let projection = WebProjection::new(registry, summary, source_commit, build)?;
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

fn predicate_name(predicate: IntPredicate) -> String {
    match predicate {
        IntPredicate::Even => "even".to_owned(),
        IntPredicate::LessThan(limit) => format!("less_than:{limit}"),
        IntPredicate::Always => "always".to_owned(),
        IntPredicate::Never => "never".to_owned(),
    }
}

#[derive(Debug)]
pub enum WebProjectionError {
    Dataset(GenerationError),
    Program(VisualProgramError),
    Json(serde_json::Error),
}

impl fmt::Display for WebProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dataset(error) => write!(formatter, "cannot generate Web dataset: {error}"),
            Self::Program(error) => write!(formatter, "cannot generate visual program: {error}"),
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

impl From<VisualProgramError> for WebProjectionError {
    fn from(error: VisualProgramError) -> Self {
        Self::Program(error)
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

    use super::{WEB_PROJECTION_FORMAT, WebBuildEnvironment, to_json};

    const REGISTRY: &str = include_str!("../../../registry/atlas.yaml");

    #[test]
    fn projection_is_deterministic_and_carries_authority_identity() {
        let registry: Registry = serde_yaml::from_str(REGISTRY).unwrap();
        let summary = summarize_registry(&registry).unwrap();

        let build = WebBuildEnvironment {
            rustc: "rustc 1.90.0 (test)",
            wasm_bindgen: "wasm-bindgen 0.2.100",
            target: "wasm32-unknown-unknown",
            profile: "release",
        };
        let first = to_json(&registry, &summary, "0123456789abcdef", build).unwrap();
        let second = to_json(&registry, &summary, "0123456789abcdef", build).unwrap();

        assert_eq!(first, second);
        let value: serde_json::Value = serde_json::from_str(&first).unwrap();
        assert_eq!(value["format"], WEB_PROJECTION_FORMAT);
        assert_eq!(value["source_commit"], "0123456789abcdef");
        assert_eq!(value["registry_digest"], summary.digest);
        assert_eq!(value["build"]["rustc"], "rustc 1.90.0 (test)");
        assert_eq!(value["build"]["wasm_bindgen"], "wasm-bindgen 0.2.100");
        assert_eq!(value["build"]["target"], "wasm32-unknown-unknown");
        assert_eq!(value["build"]["profile"], "release");
        assert_eq!(value["counts"]["conditions"], 2);
        assert_eq!(value["counts"]["problems"], 32);
        assert_eq!(value["counts"]["algorithms"], 40);
        assert_eq!(value["counts"]["implementations"], 44);
        assert_eq!(value["conditions"][0]["id"], "state.spare_capacity");
        let heap = value["algorithms"]
            .as_array()
            .unwrap()
            .iter()
            .find(|algorithm| algorithm["id"] == "priority_queue.binary_heap.push")
            .unwrap();
        assert!(heap["costs"].as_array().unwrap().iter().any(|claim| {
            claim["value"]["metric"] == "time"
                && claim["value"]["bound"] == "O(log n)"
                && claim["value"]["requires"][0] == "state.spare_capacity"
        }));
        let graph_problem = value["problems"]
            .as_array()
            .unwrap()
            .iter()
            .find(|problem| problem["id"] == "graph.nonnegative_shortest_distances")
            .unwrap();
        assert!(
            graph_problem["requires"]["value"]
                .as_array()
                .unwrap()
                .iter()
                .any(|requirement| requirement.as_str().unwrap().contains("nonnegative"))
        );
        let reservoir = value["algorithms"]
            .as_array()
            .unwrap()
            .iter()
            .find(|algorithm| algorithm["id"] == "stream.sample.reservoir_r")
            .unwrap();
        assert_eq!(reservoir["deterministic"]["value"], false);
        let top_k = value["implementations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|implementation| implementation["id"] == "stream.top_k.rust.std_binary_heap.v1")
            .unwrap();
        assert_eq!(top_k["version"]["value"], "0.1.0");
        assert_eq!(top_k["effects"]["level"], "tested");
        assert!(
            top_k["effects"]["value"]["allocation"]
                .as_str()
                .unwrap()
                .contains("at most k")
        );
        assert_eq!(value["datasets"].as_array().unwrap().len(), 10);
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
        assert_eq!(
            value["dynamics"][0]["algorithm_id"],
            "order.is_sorted.adjacent"
        );
        assert_eq!(
            value["dynamics"][0]["ast_id"],
            "ast.order.is_sorted.adjacent.v0"
        );
        assert_eq!(value["dynamics"][0]["max_interactive_input_length"], 64);
        assert_eq!(
            value["dynamics"][0]["max_analytical_trace_input_length"],
            64
        );
        assert!(
            value["dynamics"][0]["pseudocode_source"]
                .as_str()
                .unwrap()
                .contains("operation is-sorted.adjacent.compare | Compare")
        );
        assert_eq!(
            value["dynamics"][0]["program"]["format"],
            "atlas-visual-bytecode-private-v0"
        );
        assert_eq!(
            value["dynamics"][0]["program"]["instructions"]
                .as_array()
                .unwrap()
                .len(),
            9
        );
        assert_eq!(
            value["dynamics"][0]["presentation"]["result_view"],
            "sortedness"
        );
        assert_eq!(value["dynamics"][1]["algorithm_id"], "sort.insertion");
        assert_eq!(value["dynamics"][1]["ast_id"], "ast.sort.insertion.v0");
        assert_eq!(value["dynamics"][1]["max_interactive_input_length"], 64);
        assert_eq!(
            value["dynamics"][1]["max_analytical_trace_input_length"],
            32
        );
        assert!(
            value["dynamics"][1]["pseudocode_source"]
                .as_str()
                .unwrap()
                .contains("operation insertion.adjacent.swap | Swap")
        );
        assert_eq!(
            value["dynamics"][1]["program"]["instructions"]
                .as_array()
                .unwrap()
                .len(),
            13
        );
        assert_eq!(
            value["dynamics"][1]["presentation"]["result_view"],
            "stable_sorted"
        );
        assert_eq!(value["dynamics"][1]["presentation"]["tracks_origins"], true);
        assert_eq!(
            value["dynamics"][2]["algorithm_id"],
            "reverse.symmetric.in_place"
        );
        assert_eq!(
            value["dynamics"][2]["ast_id"],
            "ast.reverse.symmetric.in_place.v0"
        );
        assert_eq!(value["dynamics"][2]["max_interactive_input_length"], 64);
        assert_eq!(value["dynamics"][2]["max_analytical_trace_input_length"], 0);
        assert!(
            value["dynamics"][2]["pseudocode_source"]
                .as_str()
                .unwrap()
                .contains("operation reverse.symmetric.swap | Swap")
        );
        assert_eq!(
            value["dynamics"][2]["program"]["instructions"]
                .as_array()
                .unwrap()
                .len(),
            11
        );
        assert_eq!(
            value["dynamics"][2]["presentation"]["result_view"],
            "reversed"
        );
        assert_eq!(value["dynamics"][2]["presentation"]["tracks_origins"], true);
        assert_eq!(
            value["dynamics"][3]["algorithm_id"],
            "select.minimum.linear"
        );
        assert_eq!(
            value["dynamics"][3]["ast_id"],
            "ast.select.minimum.linear.v0"
        );
        assert_eq!(
            value["dynamics"][3]["program"]["format"],
            "atlas-visual-bytecode-private-v0"
        );
        assert_eq!(
            value["dynamics"][3]["program"]["instructions"]
                .as_array()
                .unwrap()
                .len(),
            9
        );
        assert_eq!(value["dynamics"][3]["presentation"]["key"], "minimum");
        assert_eq!(
            value["datasets"][8]["case_id"],
            "partition.adversarial.alternating"
        );
        assert_eq!(value["datasets"][8]["predicate"], "even");
        assert_eq!(
            value["dynamics"][4]["algorithm_id"],
            "partition.two_pointer.in_place"
        );
        assert_eq!(
            value["dynamics"][4]["program"]["instructions"]
                .as_array()
                .unwrap()
                .len(),
            19
        );
        assert_eq!(value["dynamics"][4]["presentation"]["key"], "partition");
        assert_eq!(
            value["dynamics"][4]["presentation"]["dataset_predicate"],
            "even"
        );
        assert!(first.contains("order.is_sorted.adjacent"));
    }
}
