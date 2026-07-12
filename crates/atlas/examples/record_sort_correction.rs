use std::env;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use atlas::datasets::SORT_BENCHMARK_SPEC;
use atlas::executions::{
    CorrectionResult, EXPERIMENTAL_EXECUTION_FORMAT, ExecutionBody, ExecutionDataset,
    ExecutionEnvironment, ExecutionMode, ExecutionParameters, ExecutionProvenance, ExecutionRecord,
    digest_i32_values,
};
use atlas::registry::load_registry;
use atlas_algorithms::insertion_sort::insertion_sort_by;

const IMPLEMENTATION_ID: &str = "sort.insertion.rust.slice.v1";
const DATASET_CASE_ID: &str = "sort.benchmark.uniform.64";
const DEFAULT_OUTPUT: &str = "build/executions/sort-insertion-uniform-64-correction.yaml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_path()?;
    let registry = load_registry(Path::new("registry/atlas.yaml"))
        .map_err(|error| format!("cannot load registry/atlas.yaml: {error}"))?;
    if !registry
        .implementations
        .iter()
        .any(|implementation| implementation.id == IMPLEMENTATION_ID)
    {
        return Err(format!("implementation {IMPLEMENTATION_ID:?} is not registered").into());
    }

    let case = SORT_BENCHMARK_SPEC
        .cases
        .iter()
        .find(|case| case.id == DATASET_CASE_ID)
        .ok_or("versioned correction recipe references an unknown dataset case")?;
    let dataset = SORT_BENCHMARK_SPEC.generate(case)?;
    let mut actual = dataset.values.clone();
    let mut expected = dataset.values.clone();
    insertion_sort_by(&mut actual, i32::cmp);
    expected.sort();

    let record = ExecutionRecord::from_body(ExecutionBody {
        format: EXPERIMENTAL_EXECUTION_FORMAT.to_owned(),
        recipe_id: "sort.insertion.uniform.64.correction.v1".to_owned(),
        mode: ExecutionMode::Correction,
        implementation_id: IMPLEMENTATION_ID.to_owned(),
        dataset: ExecutionDataset {
            spec_id: dataset.spec_id.to_owned(),
            case_id: dataset.case_id.to_owned(),
            content_digest_sha256: dataset.content_digest_sha256,
            seed: dataset.seed,
            element_count: dataset.values.len(),
        },
        parameters: ExecutionParameters {
            value_type: "i32".to_owned(),
            operation: "i32::cmp".to_owned(),
            build_profile: if cfg!(debug_assertions) {
                "debug".to_owned()
            } else {
                "release".to_owned()
            },
        },
        environment: capture_environment()?,
        result: CorrectionResult {
            passed: actual == expected,
            output_digest_sha256: digest_i32_values(&actual),
        },
        provenance: ExecutionProvenance {
            command: format!(
                "cargo run -p atlas --example record_sort_correction -- {}",
                output.display()
            ),
            recipe_source: "file:crates/atlas/examples/record_sort_correction.rs".to_owned(),
            implementation_source: "file:crates/atlas-algorithms/src/insertion_sort.rs".to_owned(),
        },
    })?;
    if !record.body.result.passed {
        return Err("correction recipe failed; no execution was written".into());
    }
    record.write_yaml(&output)?;
    println!("Wrote {} to {}", record.id, output.display());
    Ok(())
}

fn output_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let output = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT));
    if arguments.next().is_some() {
        return Err("record_sort_correction accepts at most one output path".into());
    }
    if !output.starts_with(Path::new("build/executions"))
        || output
            .components()
            .any(|component| component == Component::ParentDir)
    {
        return Err("execution output must remain under build/executions".into());
    }
    Ok(output)
}

fn capture_environment() -> Result<ExecutionEnvironment, Box<dyn std::error::Error>> {
    let rustc = command_output("rustc", &["-vV"])?;
    let compiler = rustc
        .lines()
        .next()
        .unwrap_or("rustc unavailable")
        .to_owned();
    let target = rustc
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .unwrap_or("unavailable")
        .to_owned();
    Ok(ExecutionEnvironment {
        git_commit: command_output("git", &["rev-parse", "HEAD"])?,
        git_dirty: !command_output("git", &["status", "--porcelain"])?.is_empty(),
        compiler,
        target,
    })
}

fn command_output(program: &str, arguments: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(program).args(arguments).output()?;
    if !output.status.success() {
        return Err(format!(
            "{program} exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}
