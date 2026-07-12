use atlas::datasets::SORT_BENCHMARK_SPEC;
use atlas_bench::{
    AdaptiveBenchmarkSettings, BenchmarkContext, SortImplementation, benchmark_sort_suite,
    comparability_errors,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let implementation_id = arguments
        .next()
        .ok_or("expected one sorting implementation ID")?;
    if arguments.next().is_some() {
        return Err("expected exactly one sorting implementation ID".into());
    }
    let implementation = SortImplementation::from_id(&implementation_id)?;
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    if profile != "release" {
        return Err("run this benchmark example with --release".into());
    }
    let context = BenchmarkContext::capture(profile)?;
    let case = SORT_BENCHMARK_SPEC
        .cases
        .iter()
        .find(|case| case.id == "sort.benchmark.uniform.2048")
        .expect("the benchmark campaign requires its reference dataset");
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
    let implementations = [implementation];
    let suite = benchmark_sort_suite(&dataset, &implementations, settings, context.clone())?;

    println!("context: {context:#?}");
    println!(
        "dataset: {} seed={} elements={} sha256={}",
        dataset.case_id,
        dataset.seed,
        dataset.values.len(),
        dataset.content_digest_sha256
    );
    println!("adaptive warmup rounds: {}", suite.warmup_rounds);
    println!("batch recalibrations: {}", suite.recalibrations);
    println!("requested adaptive settings: {settings:#?}");
    println!("diagnostics before: {:#?}", suite.diagnostics_before);
    println!("diagnostics after: {:#?}", suite.diagnostics_after);
    println!("diagnostic delta: {:#?}", suite.diagnostic_delta());
    for result in &suite.results {
        println!(
            "{} median={}ns/invocation mad={}ns min={}ns max={}ns samples={} invocations/sample={}",
            result.implementation_id,
            result.summary.median_ns,
            result.summary.median_absolute_deviation_ns,
            result.summary.minimum_ns,
            result.summary.maximum_ns,
            result.samples_ns.len(),
            result.invocations_per_sample,
        );
        println!("  warmup samples ns: {:?}", result.warmup_samples_ns);
        println!("  batch elapsed ns: {:?}", result.batch_elapsed_ns);
        println!("  measured samples ns: {:?}", result.samples_ns);
        println!("  execution positions: {:?}", result.sample_positions);
        for warning in &result.quality_warnings {
            println!("  WARNING: {warning}");
        }
    }
    for pair in suite.results.windows(2) {
        if !comparability_errors(&pair[0], &pair[1]).is_empty() {
            return Err("benchmark results are not comparable".into());
        }
    }
    let quality_errors = suite.quality_errors();
    if !quality_errors.is_empty() {
        return Err(format!(
            "measured series failed quality checks: {}",
            quality_errors.join(" | ")
        )
        .into());
    }
    Ok(())
}
