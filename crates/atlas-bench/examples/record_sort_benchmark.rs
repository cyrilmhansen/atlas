use std::env;
use std::path::{Component, Path, PathBuf};

use atlas::datasets::SORT_BENCHMARK_SPEC;
use atlas::registry::load_registry;
use atlas_bench::{
    AdaptiveBenchmarkSettings, BenchmarkContext, SortImplementation, benchmark_sort_suite,
    execution_record_from_benchmark,
};

const DATASET_CASE_ID: &str = "sort.benchmark.uniform.2048";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (implementation_id, output) = arguments()?;
    let implementation = SortImplementation::from_id(&implementation_id)?;
    if !cfg!(not(debug_assertions)) {
        return Err("run this benchmark recipe with --release".into());
    }
    let registry = load_registry(Path::new("registry/atlas.yaml"))
        .map_err(|error| format!("cannot load registry/atlas.yaml: {error}"))?;
    if !registry
        .implementations
        .iter()
        .any(|candidate| candidate.id == implementation_id)
    {
        return Err(format!("implementation {implementation_id:?} is not registered").into());
    }
    let case = SORT_BENCHMARK_SPEC
        .cases
        .iter()
        .find(|case| case.id == DATASET_CASE_ID)
        .ok_or("versioned benchmark recipe references an unknown dataset case")?;
    let dataset = SORT_BENCHMARK_SPEC.generate(case)?;
    let settings = AdaptiveBenchmarkSettings {
        minimum_warmup_rounds: 20,
        maximum_warmup_rounds: 100,
        stability_window: 5,
        required_stable_windows: 3,
        stability_tolerance_per_million: 50_000,
        measured_rounds: 21,
        target_sample_time_ns: 10_000_000,
        maximum_batch_memory_bytes: 64 * 1024 * 1024,
        calibration_runs: 5,
        maximum_recalibrations: 2,
    };
    let context = BenchmarkContext::capture("release")?;
    let suite = benchmark_sort_suite(&dataset, &[implementation], settings, context)?;
    let result = suite
        .results
        .first()
        .ok_or("benchmark suite returned no result")?;
    let record = execution_record_from_benchmark(
        "sort.uniform.2048.benchmark.v1",
        format!(
            "cargo run --release -p atlas-bench --example record_sort_benchmark -- {} {}",
            implementation_id,
            output.display()
        ),
        "file:crates/atlas-bench/examples/record_sort_benchmark.rs",
        implementation_source(implementation),
        &dataset,
        &suite,
        result,
    )?;
    record.write_yaml(&output)?;
    println!("Wrote {} to {}", record.id, output.display());
    Ok(())
}

fn arguments() -> Result<(String, PathBuf), Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let implementation_id = arguments
        .next()
        .ok_or("expected one sorting implementation ID")?
        .into_string()
        .map_err(|_| "implementation ID must be valid UTF-8")?;
    let output = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| default_output(&implementation_id));
    if arguments.next().is_some() {
        return Err(
            "record_sort_benchmark accepts an implementation ID and optional output path".into(),
        );
    }
    if !output.starts_with(Path::new("build/executions"))
        || output
            .components()
            .any(|component| component == Component::ParentDir)
    {
        return Err("execution output must remain under build/executions".into());
    }
    Ok((implementation_id, output))
}

fn default_output(implementation_id: &str) -> PathBuf {
    let name = implementation_id.replace('.', "-");
    PathBuf::from(format!(
        "build/executions/{name}-uniform-2048-benchmark.yaml"
    ))
}

fn implementation_source(implementation: SortImplementation) -> &'static str {
    match implementation {
        SortImplementation::MergeAllocating | SortImplementation::MergeCallerScratch => {
            "file:crates/atlas-algorithms/src/merge_sort.rs"
        }
        SortImplementation::InsertionInPlace => {
            "file:crates/atlas-algorithms/src/insertion_sort.rs"
        }
    }
}
