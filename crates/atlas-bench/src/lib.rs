use std::fmt;
use std::fs;
use std::hint::black_box;
use std::process::Command;
use std::time::Instant;

use atlas::datasets::GeneratedDataset;
use atlas_algorithms::{
    insertion_sort::insertion_sort_by,
    merge_sort::{merge_sort_by, merge_sort_by_with_scratch},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BenchmarkContext {
    pub git_commit: String,
    pub git_dirty: bool,
    pub rustc: String,
    pub target_triple: String,
    pub rustflags: String,
    pub target_arch: String,
    pub target_os: String,
    pub cpu_model: String,
    pub logical_cpus: usize,
    pub profile: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BenchmarkSettings {
    pub warmup_runs: usize,
    pub measured_runs: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SampleSummary {
    pub minimum_ns: u128,
    pub median_ns: u128,
    pub maximum_ns: u128,
    pub median_absolute_deviation_ns: u128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BenchmarkResult {
    pub implementation_id: &'static str,
    pub dataset_case_id: &'static str,
    pub dataset_digest_sha256: String,
    pub element_count: usize,
    pub seed: u64,
    pub settings: BenchmarkSettings,
    pub context: BenchmarkContext,
    pub correction_checked: bool,
    pub samples_ns: Vec<u128>,
    pub summary: SampleSummary,
    pub quality_warnings: Vec<&'static str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortImplementation {
    MergeAllocating,
    MergeCallerScratch,
    InsertionInPlace,
}

impl SortImplementation {
    pub fn id(self) -> &'static str {
        match self {
            Self::MergeAllocating => "sort.merge.rust.slice.v1",
            Self::MergeCallerScratch => "sort.merge_with_scratch.rust.slice.v1",
            Self::InsertionInPlace => "sort.insertion.rust.slice.v1",
        }
    }
}

#[derive(Debug)]
pub struct BenchmarkError(pub String);

impl fmt::Display for BenchmarkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for BenchmarkError {}

impl BenchmarkContext {
    pub fn capture(profile: impl Into<String>) -> Result<Self, BenchmarkError> {
        let rustc = command_output("rustc", &["-vV"])?;
        Ok(Self {
            git_commit: command_output("git", &["rev-parse", "HEAD"])?,
            git_dirty: !command_output("git", &["status", "--porcelain"])?.is_empty(),
            target_triple: rustc
                .lines()
                .find_map(|line| line.strip_prefix("host: "))
                .unwrap_or("unavailable")
                .to_owned(),
            rustc,
            rustflags: std::env::var("CARGO_ENCODED_RUSTFLAGS")
                .or_else(|_| std::env::var("RUSTFLAGS"))
                .unwrap_or_default(),
            target_arch: std::env::consts::ARCH.to_owned(),
            target_os: std::env::consts::OS.to_owned(),
            cpu_model: cpu_model(),
            logical_cpus: std::thread::available_parallelism()
                .map(usize::from)
                .unwrap_or(1),
            profile: profile.into(),
        })
    }
}

pub fn benchmark_sort(
    dataset: &GeneratedDataset,
    implementation: SortImplementation,
    settings: BenchmarkSettings,
    context: BenchmarkContext,
) -> Result<BenchmarkResult, BenchmarkError> {
    if dataset.problem_id != "sequence.sort" {
        return Err(BenchmarkError(format!(
            "dataset {:?} belongs to {:?}, expected sequence.sort",
            dataset.case_id, dataset.problem_id
        )));
    }
    if settings.warmup_runs == 0 || settings.measured_runs < 3 {
        return Err(BenchmarkError(
            "benchmark requires at least one warmup and three measured runs".to_owned(),
        ));
    }

    validate_correction(dataset, implementation)?;
    for _ in 0..settings.warmup_runs {
        run_once(dataset, implementation);
    }
    let mut samples_ns = Vec::with_capacity(settings.measured_runs);
    for _ in 0..settings.measured_runs {
        let elapsed = run_once(dataset, implementation);
        samples_ns.push(elapsed);
    }
    let summary = summarize(&samples_ns)?;
    let quality_warnings = quality_warnings(&samples_ns, &summary);
    Ok(BenchmarkResult {
        implementation_id: implementation.id(),
        dataset_case_id: dataset.case_id,
        dataset_digest_sha256: dataset.content_digest_sha256.clone(),
        element_count: dataset.values.len(),
        seed: dataset.seed,
        settings,
        context,
        correction_checked: true,
        samples_ns,
        summary,
        quality_warnings,
    })
}

pub fn comparability_errors(left: &BenchmarkResult, right: &BenchmarkResult) -> Vec<&'static str> {
    let mut errors = Vec::new();
    if left.context != right.context {
        errors.push("execution contexts differ");
    }
    if left.dataset_digest_sha256 != right.dataset_digest_sha256 {
        errors.push("dataset digests differ");
    }
    if left.settings != right.settings {
        errors.push("benchmark settings differ");
    }
    errors
}

pub fn summarize(samples: &[u128]) -> Result<SampleSummary, BenchmarkError> {
    if samples.is_empty() {
        return Err(BenchmarkError("cannot summarize zero samples".to_owned()));
    }
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let median_ns = median(&sorted);
    let mut deviations: Vec<_> = sorted
        .iter()
        .map(|sample| sample.abs_diff(median_ns))
        .collect();
    deviations.sort_unstable();
    Ok(SampleSummary {
        minimum_ns: sorted[0],
        median_ns,
        maximum_ns: *sorted.last().unwrap(),
        median_absolute_deviation_ns: median(&deviations),
    })
}

pub fn quality_warnings(samples: &[u128], summary: &SampleSummary) -> Vec<&'static str> {
    let mut warnings = Vec::new();
    if summary.median_absolute_deviation_ns.saturating_mul(20) > summary.median_ns {
        warnings.push("median absolute deviation exceeds 5% of median");
    }
    if samples.len() >= 6 {
        let middle = samples.len() / 2;
        let mut first = samples[..middle].to_vec();
        let mut second = samples[middle..].to_vec();
        first.sort_unstable();
        second.sort_unstable();
        let first_median = median(&first);
        let second_median = median(&second);
        if first_median.abs_diff(second_median).saturating_mul(20) > summary.median_ns {
            warnings.push("sample-series half medians differ by more than 5%");
        }
    }
    warnings
}

fn median(sorted: &[u128]) -> u128 {
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        sorted[middle - 1].saturating_add(sorted[middle]) / 2
    } else {
        sorted[middle]
    }
}

fn validate_correction(
    dataset: &GeneratedDataset,
    implementation: SortImplementation,
) -> Result<(), BenchmarkError> {
    let mut expected = dataset.values.clone();
    expected.sort();
    let mut actual = dataset.values.clone();
    execute(&mut actual, implementation);
    if actual != expected {
        return Err(BenchmarkError(format!(
            "implementation {:?} failed correction validation on {:?}",
            implementation.id(),
            dataset.case_id
        )));
    }
    Ok(())
}

fn run_once(dataset: &GeneratedDataset, implementation: SortImplementation) -> u128 {
    let mut values = dataset.values.clone();
    let mut scratch = if implementation == SortImplementation::MergeCallerScratch {
        values.clone()
    } else {
        Vec::new()
    };
    black_box(&mut values);
    black_box(&mut scratch);
    let start = Instant::now();
    match implementation {
        SortImplementation::MergeAllocating => merge_sort_by(&mut values, i32::cmp),
        SortImplementation::MergeCallerScratch => {
            merge_sort_by_with_scratch(&mut values, &mut scratch, i32::cmp).unwrap();
        }
        SortImplementation::InsertionInPlace => insertion_sort_by(&mut values, i32::cmp),
    }
    let elapsed = start.elapsed().as_nanos();
    black_box(values);
    elapsed
}

fn execute(values: &mut [i32], implementation: SortImplementation) {
    match implementation {
        SortImplementation::MergeAllocating => merge_sort_by(values, i32::cmp),
        SortImplementation::MergeCallerScratch => {
            let mut scratch = values.to_vec();
            merge_sort_by_with_scratch(values, &mut scratch, i32::cmp).unwrap();
        }
        SortImplementation::InsertionInPlace => insertion_sort_by(values, i32::cmp),
    }
}

fn command_output(program: &str, arguments: &[&str]) -> Result<String, BenchmarkError> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| BenchmarkError(format!("cannot run {program}: {error}")))?;
    if !output.status.success() {
        return Err(BenchmarkError(format!(
            "{program} exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn cpu_model() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|contents| {
            contents.lines().find_map(|line| {
                line.strip_prefix("model name\t:")
                    .map(str::trim)
                    .map(str::to_owned)
            })
        })
        .unwrap_or_else(|| "unavailable".to_owned())
}

#[cfg(test)]
mod tests {
    use atlas::datasets::SORT_BENCHMARK_SPEC;

    use super::{
        BenchmarkContext, BenchmarkSettings, SortImplementation, benchmark_sort,
        comparability_errors, quality_warnings, summarize,
    };

    fn context() -> BenchmarkContext {
        BenchmarkContext {
            git_commit: "test".to_owned(),
            git_dirty: false,
            rustc: "rustc test".to_owned(),
            target_triple: "test-triple".to_owned(),
            rustflags: String::new(),
            target_arch: "test-arch".to_owned(),
            target_os: "test-os".to_owned(),
            cpu_model: "test-cpu".to_owned(),
            logical_cpus: 1,
            profile: "test".to_owned(),
        }
    }

    #[test]
    fn summary_reports_median_and_robust_dispersion() {
        let summary = summarize(&[10, 12, 14, 100, 16]).unwrap();

        assert_eq!(summary.minimum_ns, 10);
        assert_eq!(summary.median_ns, 14);
        assert_eq!(summary.maximum_ns, 100);
        assert_eq!(summary.median_absolute_deviation_ns, 2);
    }

    #[test]
    fn all_sort_backends_pass_correction_before_measurement() {
        let dataset = SORT_BENCHMARK_SPEC
            .generate(&SORT_BENCHMARK_SPEC.cases[0])
            .unwrap();
        let settings = BenchmarkSettings {
            warmup_runs: 1,
            measured_runs: 3,
        };

        for implementation in [
            SortImplementation::MergeAllocating,
            SortImplementation::MergeCallerScratch,
            SortImplementation::InsertionInPlace,
        ] {
            let result = benchmark_sort(&dataset, implementation, settings, context()).unwrap();
            assert!(result.correction_checked);
            assert_eq!(result.samples_ns.len(), 3);
            assert_eq!(result.dataset_digest_sha256, dataset.content_digest_sha256);
        }
    }

    #[test]
    fn comparison_rejects_different_contexts() {
        let dataset = SORT_BENCHMARK_SPEC
            .generate(&SORT_BENCHMARK_SPEC.cases[0])
            .unwrap();
        let settings = BenchmarkSettings {
            warmup_runs: 1,
            measured_runs: 3,
        };
        let first = benchmark_sort(
            &dataset,
            SortImplementation::MergeAllocating,
            settings,
            context(),
        )
        .unwrap();
        let mut other_context = context();
        other_context.profile = "other".to_owned();
        let second = benchmark_sort(
            &dataset,
            SortImplementation::MergeCallerScratch,
            settings,
            other_context,
        )
        .unwrap();

        assert_eq!(
            comparability_errors(&first, &second),
            ["execution contexts differ"]
        );
    }

    #[test]
    fn quality_checks_flag_dispersion_and_series_drift() {
        let samples = [100, 102, 98, 200, 202, 198];
        let summary = summarize(&samples).unwrap();

        let warnings = quality_warnings(&samples, &summary);

        assert!(warnings.contains(&"median absolute deviation exceeds 5% of median"));
        assert!(warnings.contains(&"sample-series half medians differ by more than 5%"));
    }
}
