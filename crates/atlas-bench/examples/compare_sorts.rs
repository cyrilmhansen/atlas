use atlas::datasets::SORT_BENCHMARK_SPEC;
use atlas_bench::{
    BenchmarkContext, BenchmarkSettings, SortImplementation, benchmark_sort, comparability_errors,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    if profile != "release" {
        return Err("run this benchmark example with --release".into());
    }
    let context = BenchmarkContext::capture(profile)?;
    let dataset = SORT_BENCHMARK_SPEC.generate(&SORT_BENCHMARK_SPEC.cases[0])?;
    let settings = BenchmarkSettings {
        warmup_runs: 5,
        measured_runs: 21,
    };
    let mut results = Vec::new();
    for implementation in [
        SortImplementation::MergeAllocating,
        SortImplementation::MergeCallerScratch,
        SortImplementation::InsertionInPlace,
    ] {
        results.push(benchmark_sort(
            &dataset,
            implementation,
            settings,
            context.clone(),
        )?);
    }

    println!("context: {context:#?}");
    println!(
        "dataset: {} seed={} elements={} sha256={}",
        dataset.case_id,
        dataset.seed,
        dataset.values.len(),
        dataset.content_digest_sha256
    );
    for result in &results {
        println!(
            "{} median={}ns mad={}ns min={}ns max={}ns samples={}",
            result.implementation_id,
            result.summary.median_ns,
            result.summary.median_absolute_deviation_ns,
            result.summary.minimum_ns,
            result.summary.maximum_ns,
            result.samples_ns.len()
        );
        println!("  raw samples ns: {:?}", result.samples_ns);
        for warning in &result.quality_warnings {
            println!("  WARNING: {warning}");
        }
    }
    for pair in results.windows(2) {
        if !comparability_errors(&pair[0], &pair[1]).is_empty() {
            return Err("benchmark results are not comparable".into());
        }
    }
    Ok(())
}
