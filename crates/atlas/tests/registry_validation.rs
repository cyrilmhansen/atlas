use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use atlas::registry::{Registry, load_registry};

const VALID_REGISTRY: &str = include_str!("../../../registry/atlas.yaml");
static NEXT_DATABASE: AtomicUsize = AtomicUsize::new(0);

fn parse(contents: &str) -> Registry {
    serde_yaml::from_str(contents).expect("test fixture must match the schema shape")
}

fn expected_list(registry: &Registry, selected_kind: Option<&str>) -> String {
    let mut output = String::new();
    if selected_kind.is_none_or(|kind| kind == "problem") {
        for problem in &registry.problems {
            writeln!(&mut output, "problem\t{}", problem.id).unwrap();
        }
    }
    if selected_kind.is_none_or(|kind| kind == "algorithm") {
        for algorithm in &registry.algorithms {
            writeln!(&mut output, "algorithm\t{}", algorithm.id).unwrap();
        }
    }
    if selected_kind.is_none_or(|kind| kind == "implementation") {
        for implementation in &registry.implementations {
            writeln!(&mut output, "implementation\t{}", implementation.id).unwrap();
        }
    }
    output
}

fn workspace_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn temporary_database_path() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "atlas-index-{}-{}.sqlite3",
        std::process::id(),
        NEXT_DATABASE.fetch_add(1, Ordering::Relaxed)
    ))
}

#[test]
fn accepts_the_committed_registry() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry/atlas.yaml");
    let registry = load_registry(&path).expect("committed registry must be valid");

    assert_eq!(registry.problems.len(), 14);
    assert_eq!(registry.algorithms.len(), 19);
    assert_eq!(registry.implementations.len(), 22);
    assert!(registry.executions.is_empty());
    let linear_search = registry
        .algorithms
        .iter()
        .find(|algorithm| algorithm.id == "search.linear")
        .expect("linear search algorithm must be present");
    assert!(linear_search.stable.is_none());
    assert!(linear_search.in_place.is_none());
    assert!(linear_search.requires.is_none());
    let binary_search = registry
        .algorithms
        .iter()
        .find(|algorithm| algorithm.id == "search.binary.lower_bound")
        .expect("binary search algorithm must be present");
    assert_eq!(binary_search.requires.as_ref().unwrap().value.len(), 1);
    let sorted_merge = registry
        .problems
        .iter()
        .find(|problem| problem.id == "sequence.merge_sorted")
        .expect("sorted merge problem must be present");
    assert_eq!(sorted_merge.requires.as_ref().unwrap().value.len(), 2);
    let hash_deduplication = registry
        .algorithms
        .iter()
        .find(|algorithm| algorithm.id == "deduplicate.hash.stable")
        .expect("hash deduplication algorithm must be present");
    assert_eq!(
        hash_deduplication.time_expected.as_ref().unwrap().value,
        "O(n)"
    );
    for implementation in &registry.implementations {
        assert_eq!(
            implementation.abi.value,
            "Rust calling convention; no stable ABI"
        );
        assert!(!implementation.target.value.is_empty());
    }
    let atlas_implementations = registry
        .implementations
        .iter()
        .filter(|implementation| implementation.version.value == "0.1.0")
        .collect::<Vec<_>>();
    assert_eq!(atlas_implementations.len(), 20);
    assert!(
        atlas_implementations
            .iter()
            .all(|implementation| implementation.license.value == "MIT")
    );
    let petgraph_bfs = registry
        .implementations
        .iter()
        .find(|implementation| implementation.id == "graph.bfs.petgraph.0_8_3")
        .expect("petgraph BFS implementation must be present");
    assert_eq!(petgraph_bfs.version.value, "0.8.3");
    assert_eq!(petgraph_bfs.license.value, "MIT OR Apache-2.0");
    assert_eq!(petgraph_bfs.implements, "graph.bfs.traversal");
}

#[test]
fn committed_registry_keeps_exact_graph_contracts_separate() {
    let registry = parse(VALID_REGISTRY);

    let bfs_traversal = registry
        .algorithms
        .iter()
        .find(|algorithm| algorithm.id == "graph.bfs.traversal")
        .expect("BFS traversal must be present");
    assert_eq!(bfs_traversal.solves, "graph.reachable_traversal");

    let bfs_paths = registry
        .algorithms
        .iter()
        .find(|algorithm| algorithm.id == "graph.bfs.shortest_paths")
        .expect("BFS shortest paths must be present");
    assert_eq!(bfs_paths.solves, "graph.unweighted_shortest_paths");

    let dijkstra_distances = registry
        .algorithms
        .iter()
        .find(|algorithm| algorithm.id == "graph.dijkstra.distances")
        .expect("Dijkstra distances must be present");
    assert_eq!(
        dijkstra_distances.solves,
        "graph.nonnegative_shortest_distances"
    );

    let dijkstra_tree = registry
        .algorithms
        .iter()
        .find(|algorithm| algorithm.id == "graph.dijkstra.shortest_path_tree")
        .expect("Dijkstra shortest-path tree must be present");
    assert_eq!(dijkstra_tree.solves, "graph.nonnegative_shortest_path_tree");
}

#[test]
fn resolves_all_committed_evidence_references() {
    let registry = parse(VALID_REGISTRY);

    let errors = registry.validate_evidence(&workspace_root());

    assert!(errors.is_empty(), "unexpected errors: {errors:#?}");
}

#[test]
fn rejects_a_missing_evidence_file() {
    let registry = parse(&VALID_REGISTRY.replacen(
        "file:crates/atlas-algorithms/src/binary_search.rs",
        "file:crates/atlas-algorithms/src/missing.rs",
        1,
    ));

    let errors = registry.validate_evidence(&workspace_root());

    assert!(errors.iter().any(|error| {
        error.field == "algorithm.search.binary.lower_bound.requires.source"
            && error.message.contains("missing workspace file")
    }));
}

#[test]
fn rejects_an_unknown_evidence_implementation() {
    let registry = parse(&VALID_REGISTRY.replacen(
        "implementation:search.linear.rust.slice.v1",
        "implementation:search.linear.rust.missing",
        1,
    ));

    let errors = registry.validate_evidence(&workspace_root());

    assert!(errors.iter().any(|error| {
        error.field == "problem.sequence.search.input.source"
            && error.message.contains("unknown implementation")
    }));
}

#[test]
fn rejects_a_stale_rust_test_reference() {
    let registry = parse(&VALID_REGISTRY.replacen(
        "test:merge_sort::sorts_and_preserves_the_input_permutation",
        "test:merge_sort::renamed_test",
        1,
    ));

    let errors = registry.validate_evidence(&workspace_root());

    assert!(errors.iter().any(|error| {
        error.field == "problem.sequence.sort.ensures.source"
            && error.message.contains("was not found")
    }));
}

#[test]
fn rejects_an_unsupported_evidence_scheme() {
    let registry = parse(&VALID_REGISTRY.replacen("vision:0.1#6", "unknown:item", 1));

    let errors = registry.validate_evidence(&workspace_root());

    assert!(errors.iter().any(|error| {
        error.field == "problem.sequence.sort.input.source"
            && error.message.contains("unsupported provenance scheme")
    }));
}

#[test]
fn rejects_an_unknown_schema_version() {
    let registry =
        parse(&VALID_REGISTRY.replacen("schema_version: \"0.1\"", "schema_version: \"9.9\"", 1));

    let errors = registry.validate();

    assert!(errors.iter().any(|error| error.field == "schema_version"));
}

#[test]
fn rejects_a_broken_reference() {
    let registry =
        parse(&VALID_REGISTRY.replacen("solves: sequence.sort", "solves: sequence.missing", 1));

    let errors = registry.validate();

    assert!(
        errors
            .iter()
            .any(|error| error.field == "algorithm.sort.merge.top_down.solves")
    );
}

#[test]
fn rejects_a_duplicate_id_across_entity_kinds() {
    let registry =
        parse(&VALID_REGISTRY.replacen("id: sort.merge.top_down", "id: sequence.sort", 1));

    let errors = registry.validate();

    assert!(
        errors
            .iter()
            .any(|error| error.message.contains("duplicates"))
    );
}

#[test]
fn rejects_a_claim_without_provenance() {
    let contents = VALID_REGISTRY.replacen("      source: \"vision:0.1#6\"\n", "", 1);

    let error = serde_yaml::from_str::<Registry>(&contents)
        .expect_err("a claim without source must not match schema 0.1");

    assert!(error.to_string().contains("source"));
}

#[test]
fn rejects_algorithm_requirements_without_provenance() {
    let registry = parse(&VALID_REGISTRY.replacen(
        "source: \"file:crates/atlas-algorithms/src/binary_search.rs\"",
        "source: \"\"",
        1,
    ));

    let errors = registry.validate();

    assert!(
        errors
            .iter()
            .any(|error| error.field == "algorithm.search.binary.lower_bound.requires.source")
    );
}

#[test]
fn rejects_an_empty_algorithm_requirement_list() {
    let registry = parse(&VALID_REGISTRY.replacen(
        concat!(
            "requires:\n",
            "      value:\n",
            "        - \"input.sequence is sorted according to the comparison order\"",
        ),
        "requires:\n      value: []",
        1,
    ));

    let errors = registry.validate();

    assert!(
        errors
            .iter()
            .any(|error| { error.field == "algorithm.search.binary.lower_bound.requires.value" })
    );
}

#[test]
fn rejects_problem_requirements_without_provenance() {
    let registry = parse(&VALID_REGISTRY.replacen(
        "source: \"definition:sequence.merge_sorted\"",
        "source: \"\"",
        1,
    ));

    let errors = registry.validate();

    assert!(
        errors
            .iter()
            .any(|error| error.field == "problem.sequence.merge_sorted.requires.source")
    );
}

#[test]
fn rejects_an_empty_problem_requirement_list() {
    let registry = parse(&VALID_REGISTRY.replacen(
        concat!(
            "requires:\n",
            "      value:\n",
            "        - \"input.left is sorted according to input.order\"\n",
            "        - \"input.right is sorted according to input.order\"",
        ),
        "requires:\n      value: []",
        1,
    ));

    let errors = registry.validate();

    assert!(
        errors
            .iter()
            .any(|error| error.field == "problem.sequence.merge_sorted.requires.value")
    );
}

#[test]
fn rejects_expected_time_without_provenance() {
    let registry = parse(&VALID_REGISTRY.replacen(
        "source: \"docs:https://docs.rs/hashbrown/0.17.1/hashbrown/\"",
        "source: \"\"",
        1,
    ));

    let errors = registry.validate();

    assert!(
        errors
            .iter()
            .any(|error| error.field == "algorithm.deduplicate.hash.stable.time_expected.source")
    );
}

#[test]
fn rejects_an_implementation_without_required_metadata() {
    let contents = VALID_REGISTRY.replacen(
        concat!(
            "    version:\n",
            "      value: \"0.1.0\"\n",
            "      level: declared\n",
            "      source: \"file:crates/atlas-algorithms/Cargo.toml\"\n",
        ),
        "",
        1,
    );

    let error = serde_yaml::from_str::<Registry>(&contents)
        .expect_err("implementation version metadata must be required");

    assert!(error.to_string().contains("version"));
}

#[test]
fn rejects_implementation_metadata_without_provenance() {
    let registry = parse(&VALID_REGISTRY.replacen(
        "source: \"definition:rust-api-contract\"",
        "source: \"\"",
        1,
    ));

    let errors = registry.validate();

    assert!(
        errors
            .iter()
            .any(|error| { error.field == "implementation.sort.merge.rust.slice.v1.abi.source" })
    );
}

#[test]
fn cli_accepts_an_explicit_registry_path() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let registry = workspace.join("registry/atlas.yaml");

    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["validate", registry.to_str().expect("UTF-8 test path")])
        .output()
        .expect("atlas binary must run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Validated 14 problem(s)"));
}

#[test]
fn cli_lists_all_entity_kinds_in_manifest_order() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .arg("list")
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let registry = parse(VALID_REGISTRY);
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8 CLI output"),
        expected_list(&registry, None)
    );
}

#[test]
fn cli_filters_entities_by_kind() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["list", "algorithm"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let registry = parse(VALID_REGISTRY);
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8 CLI output"),
        expected_list(&registry, Some("algorithm"))
    );
}

#[test]
fn cli_rejects_an_unknown_entity_kind() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["list", "execution"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("expected problem, algorithm, or implementation")
    );
}

#[test]
fn cli_shows_a_problem_with_evidence() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["show", "sequence.search"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("type: problem\nid: sequence.search\n"));
    assert!(stdout.contains("ensures:\n  value:\n"));
    assert!(stdout.contains("  level: tested\n  source: tests:linear_search"));
}

#[test]
fn cli_shows_only_present_algorithm_properties() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["show", "search.linear"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("type: algorithm\nid: search.linear\nsolves: sequence.search\n"));
    assert!(stdout.contains("deterministic:\n  value: true\n"));
    assert!(!stdout.contains("\nstable:"));
    assert!(!stdout.contains("\nin_place:"));
}

#[test]
fn cli_shows_algorithm_requirements() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["show", "search.binary.lower_bound"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("solves: sequence.search\n"));
    assert!(stdout.contains(
        "requires:\n  value:\n    - input.sequence is sorted according to the comparison order\n"
    ));
    assert!(stdout.contains(
        "  level: declared\n  source: file:crates/atlas-algorithms/src/binary_search.rs"
    ));
}

#[test]
fn cli_shows_implementation_effects() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["show", "search.linear.rust.slice.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("type: implementation\n"));
    assert!(stdout.contains("implements: search.linear\n"));
    assert!(stdout.contains("    mutates: none\n"));
    assert!(stdout.contains("    allocation: none\n"));
    assert!(stdout.contains(
        "  level: declared\n  source: file:crates/atlas-algorithms/src/linear_search.rs"
    ));
}

#[test]
fn cli_rejects_an_unknown_entity_id() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["show", "missing.entity"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("entity \"missing.entity\" not found")
    );
}

#[test]
fn cli_requires_exactly_one_show_id() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .arg("show")
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("show requires an entity ID"));
}

#[test]
fn cli_searches_entity_ids() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["search", "search.binary"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8 CLI output"),
        concat!(
            "algorithm\tsearch.binary.lower_bound\n",
            "implementation\tsearch.binary.rust.slice.v1\n",
        )
    );
}

#[test]
fn cli_searches_declared_names_ignoring_case() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["search", "TOP-DOWN MERGE SORT"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8 CLI output"),
        "algorithm\tsort.merge.top_down\n"
    );
}

#[test]
fn cli_searches_both_imported_breadth_first_contracts() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["search", "breadth-first"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8 CLI output"),
        concat!(
            "algorithm\tgraph.bfs.traversal\n",
            "algorithm\tgraph.bfs.shortest_paths\n",
        )
    );
}

#[test]
fn cli_explains_the_external_petgraph_distance_specialization() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "graph.dijkstra.petgraph.0_8_3"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: graph.dijkstra.petgraph.0_8_3\n",
        "  algorithm: graph.dijkstra.distances\n",
        "  problem: graph.nonnegative_shortest_distances\n",
    )));
    assert!(
        stdout.contains(
            "entrypoint:\n  value: petgraph::algo::dijkstra specialized with goal = None\n"
        )
    );
    assert!(stdout.contains("    allocation: allocates a distance map and priority queue\n"));
    assert!(stdout.contains("every edge cost is nonnegative"));
}

#[test]
fn cli_search_with_no_match_succeeds_without_output() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["search", "not-present"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn cli_search_requires_one_non_empty_term() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for arguments in [["search"].as_slice(), ["search", ""].as_slice()] {
        let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
            .args(arguments)
            .current_dir(&workspace)
            .output()
            .expect("atlas binary must run");

        assert_eq!(output.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&output.stderr).contains("non-empty term"));
    }
}

#[test]
fn cli_qualifies_stable_non_allocating_sort_implementations() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args([
            "qualify",
            "sequence.sort",
            "--stable",
            "--allocation",
            "none",
        ])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("implementation\tsort.merge_with_scratch.rust.slice.v1\n"));
    assert!(stdout.contains("implementation\tsort.insertion.rust.slice.v1\n"));
    assert!(stdout.contains("stable\ttrue\ttested\t"));
    assert!(stdout.contains("allocation\tnone\tdeclared\t"));
    assert!(!stdout.contains("implementation\tsort.merge.rust.slice.v1\n"));
}

#[test]
fn cli_qualify_returns_no_output_when_no_implementation_matches() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["qualify", "sequence.search", "--stable"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn cli_qualify_composes_in_place_with_stability_and_allocation() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args([
            "qualify",
            "sequence.sort",
            "--stable",
            "--in-place",
            "--allocation",
            "none",
        ])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("implementation\tsort.insertion.rust.slice.v1\n"));
    assert!(!stdout.contains("sort.merge_with_scratch.rust.slice.v1"));
    assert!(stdout.contains("in_place\ttrue\tdeclared\t"));
}

#[test]
fn cli_qualify_rejects_unknown_constraints() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["qualify", "sequence.sort", "--fast"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown qualify constraint"));
}

#[test]
fn cli_replay_reports_a_missing_local_execution() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["replay", "execution.sha256.missing"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(1));
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.contains("was not found") || error.contains("generate an execution before replaying"),
        "unexpected replay error: {error}"
    );
}

#[test]
fn cli_compare_requires_two_execution_ids() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compare", "execution.sha256.one"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("at least two execution IDs"));
}

#[test]
fn cli_renders_the_experimental_cleanup_composition() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "cleanup"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("plan: sequence.cleanup.experimental.v1"));
    assert!(stdout.contains("filter.in_place.rust.vec.v1"));
    assert!(stdout.contains("copies first occurrences into output"));
    assert!(stdout.contains("rejected:\n  id: cleanup.copy_merge_hash"));
}

#[test]
fn cli_rejects_unknown_composition_scenarios() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "unknown"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown composition scenario"));
}

#[test]
fn cli_renders_a_compilable_cleanup_orchestration() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "cleanup", "--rust"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("pub fn cleanup<F>"));
    assert!(stdout.contains("filter_in_place(values, predicate)"));
    assert!(stdout.contains("deduplicate_quadratic(values)"));
}

#[test]
fn cli_selects_the_expected_time_cleanup_candidate() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "cleanup", "--goal", "expected-time"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("goal: minimize declared expected time"));
    assert!(stdout.contains("selected:\n  id: cleanup.copy_merge_hash"));
    assert!(stdout.contains("i32 implements Eq + Hash"));
    assert!(stdout.contains("rejected: insertion sort and quadratic deduplication"));
}

#[test]
fn cli_rejects_unknown_composition_options() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "cleanup", "--yaml"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown compose option"));
}

#[test]
fn cli_renders_a_compilable_expected_time_cleanup_orchestration() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "cleanup", "--goal", "expected-time", "--rust"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("pub fn cleanup_expected_time<F>"));
    assert!(stdout.contains("filter_copy(values, predicate)"));
    assert!(stdout.contains("merge_sort_by(&mut filtered, i32::cmp)"));
    assert!(stdout.contains("deduplicate_hash(&filtered)"));
}

#[test]
fn cli_renders_the_precondition_focused_find_composition() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "find"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("plan: sequence.find.experimental.v1"));
    assert!(stdout.contains("preconditions:"));
    assert!(stdout.contains("step 1 establishes the binary-search precondition"));
    assert!(stdout.contains("selected:\n  id: find.insertion_binary"));
}

#[test]
fn cli_renders_a_compilable_find_orchestration() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "find", "--rust"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("pub fn find(values: &mut [i32], needle: &i32)"));
    assert!(stdout.contains("insertion_sort_by(values, i32::cmp)"));
    assert!(stdout.contains("binary_search_by(values, needle, i32::cmp)"));
}

#[test]
fn cli_renders_the_two_input_merge_sorted_composition_and_orchestration() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let plan = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "merge-sorted"])
        .current_dir(&workspace)
        .output()
        .expect("atlas binary must run");

    assert!(plan.status.success());
    let stdout = String::from_utf8(plan.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("plan: sequence.merge_sorted.experimental.v1"));
    assert!(stdout.contains("steps 1 and 2 establish the merge preconditions"));
    assert!(stdout.contains("selected:\n  id: merge_sorted.insertion_insertion_merge"));
    assert!(stdout.contains("rejected:\n  id: merge_sorted.merge_merge_merge"));

    let source = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "merge-sorted", "--rust"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");
    assert!(source.status.success());
    let stdout = String::from_utf8(source.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("pub fn merge_after_sort(left: &mut [i32], right: &mut [i32])"));
    assert!(stdout.contains("merge_sorted_values(left, right, i32::cmp)"));
}

#[test]
fn cli_renders_the_structured_partition_sort_composition() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "partition-sort"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("plan: sequence.partition_sort.experimental.v1"));
    assert!(stdout.contains("projection.partition.matching"));
    assert!(stdout.contains("reassemble.partition"));
}

#[test]
fn cli_renders_a_compilable_partition_sort_orchestration() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "partition-sort", "--rust"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("pub fn partition_then_sort_matching"));
    assert!(stdout.contains("partition_copy(values, predicate)"));
    assert!(stdout.contains("insertion_sort_by(&mut matching, i32::cmp)"));
}

#[test]
fn cli_renders_the_unique_sort_composition_and_orchestration() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let plan = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "unique-sort"])
        .current_dir(&workspace)
        .output()
        .expect("atlas binary must run");

    assert!(plan.status.success());
    let stdout = String::from_utf8(plan.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("plan: sequence.unique_sort.experimental.v1"));
    assert!(stdout.contains("selected:\n  id: unique_sort.insertion_quadratic"));
    assert!(stdout.contains("rejected:\n  id: unique_sort.merge_hash"));

    let source = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "unique-sort", "--rust"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");
    assert!(source.status.success());
    let stdout = String::from_utf8(source.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("pub fn unique_sort(values: &mut [i32]) -> Vec<i32>"));
    assert!(stdout.contains("deduplicate_quadratic(values)"));
}

#[test]
fn cli_forces_or_forbids_implementations_without_changing_the_registry() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for arguments in [
        ["compose", "cleanup", "--force", "sort.merge.rust.slice.v1"],
        [
            "compose",
            "cleanup",
            "--forbid",
            "filter.in_place.rust.vec.v1",
        ],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
            .args(arguments)
            .current_dir(&workspace)
            .output()
            .expect("atlas binary must run");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
        assert!(stdout.contains("selected:\n  id: cleanup.copy_merge_hash"));
        assert!(stdout.contains("explicit"));
    }
}

#[test]
fn cli_rejects_constraints_that_remove_every_find_candidate() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["compose", "find", "--forbid", "search.binary.rust.slice.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("removes every"));
}

#[test]
fn cli_explains_binary_search_chain_with_requirements_and_effects() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "search.binary.rust.slice.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: search.binary.rust.slice.v1\n",
        "  algorithm: search.binary.lower_bound\n",
        "  problem: sequence.search\n",
    )));
    assert!(stdout.contains("    allocation: none\n"));
    assert!(stdout.contains(
        "requires:\n  value:\n    - input.sequence is sorted according to the comparison order\n"
    ));
    assert!(stdout.contains("problem contract:\ntype: problem\nid: sequence.search\n"));
}

#[test]
fn cli_explains_merge_sort_chain_without_extra_requirements() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "sort.merge.rust.slice.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains("  algorithm: sort.merge.top_down\n"));
    assert!(stdout.contains("  problem: sequence.sort\n"));
    assert!(stdout.contains("stable:\n  value: true\n"));
    assert!(stdout.contains("allocation: one auxiliary Vec<T> of input length for n >= 2\n"));
    assert!(!stdout.contains("\nrequires:"));
}

#[test]
fn cli_explains_merge_sort_with_caller_scratch() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "sort.merge_with_scratch.rust.slice.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: sort.merge_with_scratch.rust.slice.v1\n",
        "  algorithm: sort.merge.top_down\n",
        "  problem: sequence.sort\n",
    )));
    assert!(stdout.contains("    mutates: input.sequence, scratch.sequence\n"));
    assert!(stdout.contains("    allocation: none\n"));
    assert!(stdout.contains("Result<(), ScratchTooSmall>"));
    assert!(stdout.contains("stable:\n  value: true\n"));
}

#[test]
fn cli_explains_minimum_chain() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "select.minimum.linear.rust.slice.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: select.minimum.linear.rust.slice.v1\n",
        "  algorithm: select.minimum.linear\n",
        "  problem: sequence.minimum\n",
    )));
    assert!(stdout.contains("entrypoint:\n  value: atlas_algorithms::minimum::minimum_by\n"));
    assert!(stdout.contains("    allocation: none\n"));
    assert!(stdout.contains("result is none if and only if input.sequence is empty"));
}

#[test]
fn cli_explains_maximum_chain() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "select.maximum.linear.rust.slice.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: select.maximum.linear.rust.slice.v1\n",
        "  algorithm: select.maximum.linear\n",
        "  problem: sequence.maximum\n",
    )));
    assert!(stdout.contains("entrypoint:\n  value: atlas_algorithms::maximum::maximum_by\n"));
    assert!(stdout.contains("    allocation: none\n"));
    assert!(stdout.contains("result is the first element no less than any input element"));
}

#[test]
fn cli_explains_caller_provided_filter_output() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "filter.copy_into.rust.vec.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: filter.copy_into.rust.vec.v1\n",
        "  algorithm: filter.copy.stable\n",
        "  problem: sequence.filter\n",
    )));
    assert!(stdout.contains("    mutates: output.sequence\n"));
    assert!(
        stdout.contains("    allocation: reuses output capacity; may grow when insufficient\n")
    );
    assert!(stdout.contains("stable:\n  value: true\n"));
}

#[test]
fn cli_explains_caller_provided_partition_outputs() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "partition.copy_into.rust.vec.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: partition.copy_into.rust.vec.v1\n",
        "  algorithm: partition.copy.stable\n",
        "  problem: sequence.partition\n",
    )));
    assert!(stdout.contains("    mutates: output.matching, output.rejected\n"));
    assert!(stdout.contains(
        "    allocation: reuses both output capacities; either may grow when insufficient\n"
    ));
    assert!(stdout.contains("stable:\n  value: true\n"));
}

#[test]
fn cli_explains_in_place_reversal() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "reverse.symmetric.rust.slice.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: reverse.symmetric.rust.slice.v1\n",
        "  algorithm: reverse.symmetric.in_place\n",
        "  problem: sequence.reverse\n",
    )));
    assert!(stdout.contains("    mutates: input.sequence\n"));
    assert!(stdout.contains("    allocation: none\n"));
    assert!(stdout.contains("in_place:\n  value: true\n"));
    assert!(stdout.contains("reversing result restores input.sequence"));
}

#[test]
fn cli_explains_sorted_merge_with_problem_requirements() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "merge.sorted_into.rust.vec.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: merge.sorted_into.rust.vec.v1\n",
        "  algorithm: merge.sorted.two_way\n",
        "  problem: sequence.merge_sorted\n",
    )));
    assert!(stdout.contains("    mutates: output.sequence\n"));
    assert!(
        stdout.contains("    allocation: reuses output capacity; may grow when insufficient\n")
    );
    assert!(stdout.contains("stable:\n  value: true\n"));
    assert!(stdout.contains("    - input.left is sorted according to input.order\n"));
    assert!(stdout.contains("    - input.right is sorted according to input.order\n"));
}

#[test]
fn cli_explains_adjacent_sortedness_scan() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "order.is_sorted.rust.slice.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: order.is_sorted.rust.slice.v1\n",
        "  algorithm: order.is_sorted.adjacent\n",
        "  problem: sequence.is_sorted\n",
    )));
    assert!(stdout.contains("entrypoint:\n  value: atlas_algorithms::is_sorted::is_sorted_by\n"));
    assert!(stdout.contains("    mutates: none\n"));
    assert!(stdout.contains("    allocation: none\n"));
    assert!(stdout.contains("result is true if and only if no adjacent pair is decreasing"));
}

#[test]
fn cli_explains_hash_deduplication_costs_and_effects() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "deduplicate.hash_into.rust.vec.v1"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 CLI output");
    assert!(stdout.contains(concat!(
        "chain:\n",
        "  implementation: deduplicate.hash_into.rust.vec.v1\n",
        "  algorithm: deduplicate.hash.stable\n",
        "  problem: sequence.deduplicate\n",
    )));
    assert!(stdout.contains(
        "    allocation: allocates an internal HashSet<&T>; reuses output capacity and may grow\n"
    ));
    assert!(stdout.contains("    - T implements Eq and Hash consistently\n"));
    assert!(stdout.contains("time_worst:\n  value: O(n^2)\n"));
    assert!(stdout.contains("time_expected:\n  value: O(n)\n"));
    assert!(stdout.contains("result retains the first input occurrence and relative order"));
}

#[test]
fn cli_explain_rejects_a_non_implementation_id() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["explain", "search.binary.lower_bound"])
        .current_dir(workspace)
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("implementation \"search.binary.lower_bound\" not found")
    );
}

#[test]
fn cli_explain_requires_exactly_one_implementation_id() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for arguments in [["explain"].as_slice(), ["explain", "one", "two"].as_slice()] {
        let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
            .args(arguments)
            .current_dir(&workspace)
            .output()
            .expect("atlas binary must run");

        assert_eq!(output.status.code(), Some(2));
    }
}

#[test]
fn cli_rebuilds_a_deterministic_sqlite_index() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let database = temporary_database_path();
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_atlas"))
            .args(["index", database.to_str().expect("UTF-8 test path")])
            .current_dir(&workspace)
            .output()
            .expect("atlas binary must run")
    };

    let first = run();
    assert!(first.status.success());
    let first_stdout = String::from_utf8(first.stdout).expect("UTF-8 CLI output");
    assert!(first_stdout.contains("Indexed 55 entities, 41 relations,"));
    let first_digest = first_stdout
        .lines()
        .find(|line| line.starts_with("Logical SHA-256: "))
        .expect("digest output")
        .to_owned();

    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute(
            "INSERT INTO entities(id, kind, ordinal) VALUES ('stale', 'problem', 999)",
            [],
        )
        .unwrap();
    drop(connection);

    let second = run();
    assert!(second.status.success());
    let second_stdout = String::from_utf8(second.stdout).expect("UTF-8 CLI output");
    assert!(second_stdout.contains(&first_digest));

    let connection = rusqlite::Connection::open(&database).unwrap();
    let entities: i64 = connection
        .query_row("SELECT COUNT(*) FROM entities", [], |row| row.get(0))
        .unwrap();
    let stale: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM entities WHERE id = 'stale'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(entities, 55);
    assert_eq!(stale, 0);
    drop(connection);
    fs::remove_file(database).unwrap();
}

#[test]
fn cli_index_rejects_extra_arguments() {
    let output = Command::new(env!("CARGO_BIN_EXE_atlas"))
        .args(["index", "one.sqlite3", "two.sqlite3"])
        .output()
        .expect("atlas binary must run");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("at most one database path"));
}
