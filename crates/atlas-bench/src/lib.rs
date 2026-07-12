use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::hint::black_box;
use std::process::Command;
use std::time::Instant;

use atlas::{
    datasets::GeneratedDataset,
    executions::{
        BenchmarkRawSamples as ExecutionBenchmarkRawSamples,
        BenchmarkResult as ExecutionBenchmarkResult, BenchmarkSummary as ExecutionBenchmarkSummary,
        EXPERIMENTAL_EXECUTION_FORMAT, ExecutionBody, ExecutionDataset, ExecutionEnvironment,
        ExecutionMode, ExecutionParameters, ExecutionProvenance, ExecutionRecord, ExecutionResult,
    },
};
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdaptiveBenchmarkSettings {
    pub minimum_warmup_rounds: usize,
    pub maximum_warmup_rounds: usize,
    pub stability_window: usize,
    pub required_stable_windows: usize,
    pub stability_tolerance_per_million: u32,
    pub measured_rounds: usize,
    pub target_sample_time_ns: u128,
    pub maximum_batch_memory_bytes: usize,
    pub calibration_runs: usize,
    pub maximum_recalibrations: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BenchmarkSuite {
    pub warmup_rounds: usize,
    pub recalibrations: usize,
    pub diagnostics_before: SystemDiagnostics,
    pub diagnostics_after: SystemDiagnostics,
    pub results: Vec<BenchmarkResult>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemDiagnostics {
    pub load_average: Option<[String; 3]>,
    pub allowed_cpus: Option<String>,
    pub voluntary_context_switches: Option<u64>,
    pub nonvoluntary_context_switches: Option<u64>,
    pub scheduler_migrations: Option<u64>,
    pub minor_page_faults: Option<u64>,
    pub major_page_faults: Option<u64>,
    pub scaling_governors: Vec<String>,
    pub minimum_observed_frequency_khz: Option<u64>,
    pub maximum_observed_frequency_khz: Option<u64>,
    pub resident_memory_kib: Option<u64>,
    pub peak_resident_memory_kib: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticDelta {
    pub voluntary_context_switches: Option<u64>,
    pub nonvoluntary_context_switches: Option<u64>,
    pub scheduler_migrations: Option<u64>,
    pub minor_page_faults: Option<u64>,
    pub major_page_faults: Option<u64>,
}

impl BenchmarkSuite {
    pub fn quality_errors(&self) -> Vec<String> {
        self.results
            .iter()
            .filter(|result| !result.quality_warnings.is_empty())
            .map(|result| {
                format!(
                    "{}: {}",
                    result.implementation_id,
                    result.quality_warnings.join("; ")
                )
            })
            .collect()
    }

    pub fn diagnostic_delta(&self) -> DiagnosticDelta {
        DiagnosticDelta {
            voluntary_context_switches: counter_delta(
                self.diagnostics_before.voluntary_context_switches,
                self.diagnostics_after.voluntary_context_switches,
            ),
            nonvoluntary_context_switches: counter_delta(
                self.diagnostics_before.nonvoluntary_context_switches,
                self.diagnostics_after.nonvoluntary_context_switches,
            ),
            scheduler_migrations: counter_delta(
                self.diagnostics_before.scheduler_migrations,
                self.diagnostics_after.scheduler_migrations,
            ),
            minor_page_faults: counter_delta(
                self.diagnostics_before.minor_page_faults,
                self.diagnostics_after.minor_page_faults,
            ),
            major_page_faults: counter_delta(
                self.diagnostics_before.major_page_faults,
                self.diagnostics_after.major_page_faults,
            ),
        }
    }
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
    pub adaptive_settings: Option<AdaptiveBenchmarkSettings>,
    pub context: BenchmarkContext,
    pub correction_checked: bool,
    pub invocations_per_sample: usize,
    pub warmup_samples_ns: Vec<u128>,
    pub batch_elapsed_ns: Vec<u128>,
    pub sample_positions: Vec<usize>,
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

    pub fn from_id(id: &str) -> Result<Self, BenchmarkError> {
        match id {
            "sort.merge.rust.slice.v1" => Ok(Self::MergeAllocating),
            "sort.merge_with_scratch.rust.slice.v1" => Ok(Self::MergeCallerScratch),
            "sort.insertion.rust.slice.v1" => Ok(Self::InsertionInPlace),
            _ => Err(BenchmarkError(format!(
                "unknown sorting implementation {id:?}"
            ))),
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

pub fn execution_record_from_benchmark(
    recipe_id: &str,
    command: String,
    recipe_source: &str,
    implementation_source: &str,
    dataset: &GeneratedDataset,
    suite: &BenchmarkSuite,
    result: &BenchmarkResult,
) -> Result<ExecutionRecord, BenchmarkError> {
    if !result.quality_warnings.is_empty() {
        return Err(BenchmarkError(format!(
            "refusing unqualified benchmark observation: {}",
            result.quality_warnings.join("; ")
        )));
    }
    if result.dataset_case_id != dataset.case_id
        || result.dataset_digest_sha256 != dataset.content_digest_sha256
        || result.element_count != dataset.values.len()
        || result.seed != dataset.seed
    {
        return Err(BenchmarkError(
            "benchmark result does not match its generated dataset".to_owned(),
        ));
    }
    let requested_protocol = result.adaptive_settings.ok_or_else(|| {
        BenchmarkError("benchmark observation requires adaptive settings".to_owned())
    })?;
    let context = benchmark_context_fields(&result.context);
    let record = ExecutionRecord::from_body(ExecutionBody {
        format: EXPERIMENTAL_EXECUTION_FORMAT.to_owned(),
        recipe_id: recipe_id.to_owned(),
        mode: ExecutionMode::Benchmark,
        implementation_id: result.implementation_id.to_owned(),
        dataset: ExecutionDataset {
            spec_id: dataset.spec_id.to_owned(),
            case_id: dataset.case_id.to_owned(),
            content_digest_sha256: dataset.content_digest_sha256.clone(),
            seed: dataset.seed,
            element_count: dataset.values.len(),
        },
        parameters: ExecutionParameters {
            value_type: "i32".to_owned(),
            operation: "sort using i32::cmp".to_owned(),
            build_profile: result.context.profile.clone(),
        },
        environment: ExecutionEnvironment {
            git_commit: result.context.git_commit.clone(),
            git_dirty: result.context.git_dirty,
            compiler: result.context.rustc.clone(),
            target: result.context.target_triple.clone(),
        },
        result: ExecutionResult::Benchmark(Box::new(ExecutionBenchmarkResult {
            qualified: true,
            quality_warnings: Vec::new(),
            context,
            requested_protocol: BTreeMap::from([
                (
                    "minimum_warmup_rounds".to_owned(),
                    requested_protocol.minimum_warmup_rounds.to_string(),
                ),
                (
                    "maximum_warmup_rounds".to_owned(),
                    requested_protocol.maximum_warmup_rounds.to_string(),
                ),
                (
                    "stability_window".to_owned(),
                    requested_protocol.stability_window.to_string(),
                ),
                (
                    "required_stable_windows".to_owned(),
                    requested_protocol.required_stable_windows.to_string(),
                ),
                (
                    "stability_tolerance_per_million".to_owned(),
                    requested_protocol
                        .stability_tolerance_per_million
                        .to_string(),
                ),
                (
                    "measured_rounds".to_owned(),
                    requested_protocol.measured_rounds.to_string(),
                ),
                (
                    "target_sample_time_ns".to_owned(),
                    requested_protocol.target_sample_time_ns.to_string(),
                ),
                (
                    "maximum_batch_memory_bytes".to_owned(),
                    requested_protocol.maximum_batch_memory_bytes.to_string(),
                ),
                (
                    "calibration_runs".to_owned(),
                    requested_protocol.calibration_runs.to_string(),
                ),
                (
                    "maximum_recalibrations".to_owned(),
                    requested_protocol.maximum_recalibrations.to_string(),
                ),
            ]),
            observed: BTreeMap::from([
                ("warmup_rounds".to_owned(), suite.warmup_rounds.to_string()),
                (
                    "recalibrations".to_owned(),
                    suite.recalibrations.to_string(),
                ),
                (
                    "invocations_per_sample".to_owned(),
                    result.invocations_per_sample.to_string(),
                ),
                (
                    "correction_checked".to_owned(),
                    result.correction_checked.to_string(),
                ),
            ]),
            raw_samples: ExecutionBenchmarkRawSamples {
                warmup_ns: result
                    .warmup_samples_ns
                    .iter()
                    .map(u128::to_string)
                    .collect(),
                batch_elapsed_ns: result
                    .batch_elapsed_ns
                    .iter()
                    .map(u128::to_string)
                    .collect(),
                normalized_ns: result.samples_ns.iter().map(u128::to_string).collect(),
                execution_positions: result.sample_positions.clone(),
            },
            summary: ExecutionBenchmarkSummary {
                minimum_ns: result.summary.minimum_ns.to_string(),
                median_ns: result.summary.median_ns.to_string(),
                maximum_ns: result.summary.maximum_ns.to_string(),
                median_absolute_deviation_ns: result
                    .summary
                    .median_absolute_deviation_ns
                    .to_string(),
            },
            metrics: BTreeMap::from([
                (
                    "process_resident_memory_kib".to_owned(),
                    optional_counter(suite.diagnostics_after.resident_memory_kib),
                ),
                (
                    "process_peak_resident_memory_kib".to_owned(),
                    optional_counter(suite.diagnostics_after.peak_resident_memory_kib),
                ),
                ("algorithm_allocations".to_owned(), "unavailable".to_owned()),
                ("elements_traversed".to_owned(), "unavailable".to_owned()),
            ]),
            diagnostics_before: diagnostic_fields(&suite.diagnostics_before),
            diagnostics_after: diagnostic_fields(&suite.diagnostics_after),
            diagnostic_delta: diagnostic_delta_fields(&suite.diagnostic_delta()),
        })),
        provenance: ExecutionProvenance {
            command,
            recipe_source: recipe_source.to_owned(),
            implementation_source: implementation_source.to_owned(),
        },
    })
    .map_err(|error| BenchmarkError(error.to_string()))?;
    Ok(record)
}

fn benchmark_context_fields(context: &BenchmarkContext) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("cpu_model".to_owned(), context.cpu_model.clone()),
        ("logical_cpus".to_owned(), context.logical_cpus.to_string()),
        ("rustflags".to_owned(), context.rustflags.clone()),
        ("target_arch".to_owned(), context.target_arch.clone()),
        ("target_os".to_owned(), context.target_os.clone()),
    ])
}

fn diagnostic_fields(diagnostics: &SystemDiagnostics) -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "allowed_cpus".to_owned(),
            diagnostics
                .allowed_cpus
                .clone()
                .unwrap_or_else(|| "unavailable".to_owned()),
        ),
        (
            "load_average".to_owned(),
            diagnostics
                .load_average
                .as_ref()
                .map(|values| values.join(","))
                .unwrap_or_else(|| "unavailable".to_owned()),
        ),
        (
            "voluntary_context_switches".to_owned(),
            optional_counter(diagnostics.voluntary_context_switches),
        ),
        (
            "nonvoluntary_context_switches".to_owned(),
            optional_counter(diagnostics.nonvoluntary_context_switches),
        ),
        (
            "scheduler_migrations".to_owned(),
            optional_counter(diagnostics.scheduler_migrations),
        ),
        (
            "minor_page_faults".to_owned(),
            optional_counter(diagnostics.minor_page_faults),
        ),
        (
            "major_page_faults".to_owned(),
            optional_counter(diagnostics.major_page_faults),
        ),
        (
            "scaling_governors".to_owned(),
            if diagnostics.scaling_governors.is_empty() {
                "unavailable".to_owned()
            } else {
                diagnostics.scaling_governors.join(",")
            },
        ),
        (
            "minimum_observed_frequency_khz".to_owned(),
            optional_counter(diagnostics.minimum_observed_frequency_khz),
        ),
        (
            "maximum_observed_frequency_khz".to_owned(),
            optional_counter(diagnostics.maximum_observed_frequency_khz),
        ),
        (
            "resident_memory_kib".to_owned(),
            optional_counter(diagnostics.resident_memory_kib),
        ),
        (
            "peak_resident_memory_kib".to_owned(),
            optional_counter(diagnostics.peak_resident_memory_kib),
        ),
    ])
}

fn diagnostic_delta_fields(delta: &DiagnosticDelta) -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "voluntary_context_switches".to_owned(),
            optional_counter(delta.voluntary_context_switches),
        ),
        (
            "nonvoluntary_context_switches".to_owned(),
            optional_counter(delta.nonvoluntary_context_switches),
        ),
        (
            "scheduler_migrations".to_owned(),
            optional_counter(delta.scheduler_migrations),
        ),
        (
            "minor_page_faults".to_owned(),
            optional_counter(delta.minor_page_faults),
        ),
        (
            "major_page_faults".to_owned(),
            optional_counter(delta.major_page_faults),
        ),
    ])
}

fn optional_counter(value: Option<u64>) -> String {
    value.map_or_else(|| "unavailable".to_owned(), |value| value.to_string())
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
        adaptive_settings: None,
        context,
        correction_checked: true,
        invocations_per_sample: 1,
        warmup_samples_ns: Vec::new(),
        batch_elapsed_ns: samples_ns.clone(),
        sample_positions: vec![0; samples_ns.len()],
        samples_ns,
        summary,
        quality_warnings,
    })
}

pub fn benchmark_sort_suite(
    dataset: &GeneratedDataset,
    implementations: &[SortImplementation],
    settings: AdaptiveBenchmarkSettings,
    context: BenchmarkContext,
) -> Result<BenchmarkSuite, BenchmarkError> {
    validate_adaptive_settings(implementations, settings)?;
    let diagnostics_before = capture_system_diagnostics();
    for implementation in implementations {
        validate_correction(dataset, *implementation)?;
    }
    let mut invocations_per_sample: Vec<_> = implementations
        .iter()
        .map(|implementation| calibrate_batch(dataset, *implementation, settings))
        .collect::<Result<_, _>>()?;
    let mut batches: Vec<_> = implementations
        .iter()
        .zip(&invocations_per_sample)
        .map(|(implementation, invocations)| {
            PreparedBatch::new(dataset, *implementation, *invocations)
        })
        .collect();

    let mut warmups = vec![Vec::new(); implementations.len()];
    let mut stable_counts = vec![0_usize; implementations.len()];
    let mut warmup_rounds = 0;
    let mut recalibrations = 0;
    while warmup_rounds < settings.maximum_warmup_rounds {
        for index in rotated_indices(implementations.len(), warmup_rounds) {
            let (_, normalized) = batches[index].measure(dataset);
            warmups[index].push(normalized);
        }
        warmup_rounds += 1;
        if warmups[0].len() < settings.minimum_warmup_rounds {
            continue;
        }
        for (index, samples) in warmups.iter().enumerate() {
            if recent_windows_are_stable(
                samples,
                settings.stability_window,
                settings.stability_tolerance_per_million,
            ) {
                stable_counts[index] += 1;
            } else {
                stable_counts[index] = 0;
            }
        }
        if stable_counts
            .iter()
            .all(|count| *count >= settings.required_stable_windows)
        {
            if recalibrations < settings.maximum_recalibrations {
                let recalibrated: Vec<_> = implementations
                    .iter()
                    .enumerate()
                    .map(|(index, implementation)| {
                        let mut samples = warmups[index].clone();
                        samples.sort_unstable();
                        let buffers = if *implementation == SortImplementation::MergeCallerScratch {
                            2
                        } else {
                            1
                        };
                        let bytes = dataset
                            .values
                            .len()
                            .saturating_mul(std::mem::size_of::<i32>())
                            .saturating_mul(buffers);
                        calibrated_invocations(
                            median(&samples),
                            settings.target_sample_time_ns,
                            bytes,
                            settings.maximum_batch_memory_bytes,
                        )
                        .expect("initial calibration proved one input fits")
                    })
                    .collect::<Vec<_>>();
                let changed = recalibrated
                    .iter()
                    .zip(&invocations_per_sample)
                    .any(|(new, old)| new.abs_diff(*old).saturating_mul(10) > *old);
                if changed {
                    invocations_per_sample = recalibrated;
                    batches = implementations
                        .iter()
                        .zip(&invocations_per_sample)
                        .map(|(implementation, invocations)| {
                            PreparedBatch::new(dataset, *implementation, *invocations)
                        })
                        .collect();
                    for samples in &mut warmups {
                        samples.clear();
                    }
                    stable_counts.fill(0);
                    recalibrations += 1;
                    continue;
                }
            }
            break;
        }
    }
    if stable_counts
        .iter()
        .any(|count| *count < settings.required_stable_windows)
    {
        let unstable: Vec<_> = implementations
            .iter()
            .enumerate()
            .filter(|(index, _)| stable_counts[*index] < settings.required_stable_windows)
            .map(|(index, implementation)| {
                let start = warmups[index]
                    .len()
                    .saturating_sub(settings.stability_window * 2);
                format!(
                    "{} invocations={} recent={:?}",
                    implementation.id(),
                    invocations_per_sample[index],
                    &warmups[index][start..]
                )
            })
            .collect();
        return Err(BenchmarkError(format!(
            "adaptive warmup did not stabilize within {} rounds after {} recalibration(s): {}; diagnostics before={diagnostics_before:?}; diagnostics after={:?}",
            settings.maximum_warmup_rounds,
            recalibrations,
            unstable.join(", "),
            capture_system_diagnostics(),
        )));
    }

    let mut measured = vec![Vec::with_capacity(settings.measured_rounds); implementations.len()];
    let mut batch_elapsed =
        vec![Vec::with_capacity(settings.measured_rounds); implementations.len()];
    let mut sample_positions =
        vec![Vec::with_capacity(settings.measured_rounds); implementations.len()];
    for round in 0..settings.measured_rounds {
        for (position, index) in
            rotated_indices(implementations.len(), warmup_rounds + round).enumerate()
        {
            let (elapsed, normalized) = batches[index].measure(dataset);
            batch_elapsed[index].push(elapsed);
            measured[index].push(normalized);
            sample_positions[index].push(position);
        }
    }
    let result_settings = BenchmarkSettings {
        warmup_runs: warmup_rounds,
        measured_runs: settings.measured_rounds,
    };
    let mut results = Vec::with_capacity(implementations.len());
    for (index, implementation) in implementations.iter().enumerate() {
        let summary = summarize(&measured[index])?;
        let mut warnings = quality_warnings(&measured[index], &summary);
        if position_bias_exceeds(
            &measured[index],
            &sample_positions[index],
            settings.stability_tolerance_per_million,
        ) {
            warnings.push("sample medians differ by execution position");
        }
        results.push(BenchmarkResult {
            implementation_id: implementation.id(),
            dataset_case_id: dataset.case_id,
            dataset_digest_sha256: dataset.content_digest_sha256.clone(),
            element_count: dataset.values.len(),
            seed: dataset.seed,
            settings: result_settings,
            adaptive_settings: Some(settings),
            context: context.clone(),
            correction_checked: true,
            invocations_per_sample: invocations_per_sample[index],
            warmup_samples_ns: warmups[index].clone(),
            batch_elapsed_ns: batch_elapsed[index].clone(),
            sample_positions: sample_positions[index].clone(),
            quality_warnings: warnings,
            samples_ns: measured[index].clone(),
            summary,
        });
    }
    let diagnostics_after = capture_system_diagnostics();
    Ok(BenchmarkSuite {
        warmup_rounds,
        recalibrations,
        diagnostics_before,
        diagnostics_after,
        results,
    })
}

pub fn capture_system_diagnostics() -> SystemDiagnostics {
    let status = fs::read_to_string("/proc/self/status").ok();
    let scheduler = fs::read_to_string("/proc/self/sched").ok();
    let process_stat = fs::read_to_string("/proc/self/stat").ok();
    let load_average = fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|contents| {
            let mut fields = contents.split_whitespace();
            Some([
                fields.next()?.to_owned(),
                fields.next()?.to_owned(),
                fields.next()?.to_owned(),
            ])
        });
    let allowed_cpus = status
        .as_deref()
        .and_then(|contents| proc_value(contents, "Cpus_allowed_list"));
    let cpus = allowed_cpus
        .map(parse_cpu_list)
        .filter(|cpus| !cpus.is_empty())
        .unwrap_or_else(|| (0..256).collect());
    let mut governors = BTreeSet::new();
    let mut frequencies = Vec::new();
    for cpu in cpus {
        let root = format!("/sys/devices/system/cpu/cpu{cpu}/cpufreq");
        if let Ok(governor) = fs::read_to_string(format!("{root}/scaling_governor")) {
            governors.insert(governor.trim().to_owned());
        }
        if let Ok(frequency) = fs::read_to_string(format!("{root}/scaling_cur_freq"))
            && let Ok(frequency) = frequency.trim().parse()
        {
            frequencies.push(frequency);
        }
    }
    SystemDiagnostics {
        load_average,
        allowed_cpus: allowed_cpus.map(str::to_owned),
        voluntary_context_switches: status.as_deref().and_then(|contents| {
            proc_value(contents, "voluntary_ctxt_switches").and_then(|value| value.parse().ok())
        }),
        nonvoluntary_context_switches: status.as_deref().and_then(|contents| {
            proc_value(contents, "nonvoluntary_ctxt_switches").and_then(|value| value.parse().ok())
        }),
        scheduler_migrations: scheduler.as_deref().and_then(|contents| {
            proc_value(contents, "se.nr_migrations").and_then(|value| value.parse().ok())
        }),
        minor_page_faults: process_stat
            .as_deref()
            .and_then(|contents| process_stat_counter(contents, 10)),
        major_page_faults: process_stat
            .as_deref()
            .and_then(|contents| process_stat_counter(contents, 12)),
        scaling_governors: governors.into_iter().collect(),
        minimum_observed_frequency_khz: frequencies.iter().min().copied(),
        maximum_observed_frequency_khz: frequencies.iter().max().copied(),
        resident_memory_kib: status
            .as_deref()
            .and_then(|contents| proc_memory_kib(contents, "VmRSS")),
        peak_resident_memory_kib: status
            .as_deref()
            .and_then(|contents| proc_memory_kib(contents, "VmHWM")),
    }
}

fn process_stat_counter(contents: &str, field_number: usize) -> Option<u64> {
    let after_name = contents.rsplit_once(") ")?.1;
    after_name
        .split_whitespace()
        .nth(field_number.checked_sub(3)?)?
        .parse()
        .ok()
}

fn proc_value<'a>(contents: &'a str, key: &str) -> Option<&'a str> {
    contents.lines().find_map(|line| {
        let (candidate, value) = line.split_once(':')?;
        (candidate.trim() == key).then(|| value.trim())
    })
}

fn proc_memory_kib(contents: &str, key: &str) -> Option<u64> {
    proc_value(contents, key)?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

fn parse_cpu_list(value: &str) -> Vec<usize> {
    let mut cpus = Vec::new();
    for part in value.split(',') {
        if let Some((start, end)) = part.split_once('-') {
            if let (Ok(start), Ok(end)) = (start.parse::<usize>(), end.parse::<usize>()) {
                cpus.extend(start..=end);
            }
        } else if let Ok(cpu) = part.parse() {
            cpus.push(cpu);
        }
    }
    cpus
}

fn counter_delta(before: Option<u64>, after: Option<u64>) -> Option<u64> {
    Some(after?.saturating_sub(before?))
}

fn validate_adaptive_settings(
    implementations: &[SortImplementation],
    settings: AdaptiveBenchmarkSettings,
) -> Result<(), BenchmarkError> {
    if implementations.is_empty() {
        return Err(BenchmarkError(
            "adaptive benchmark requires at least one implementation".to_owned(),
        ));
    }
    if settings.stability_window < 2
        || settings.required_stable_windows == 0
        || settings.minimum_warmup_rounds < settings.stability_window * 2
        || settings.maximum_warmup_rounds < settings.minimum_warmup_rounds
        || settings.measured_rounds < 3
        || settings.target_sample_time_ns == 0
        || settings.maximum_batch_memory_bytes == 0
        || settings.calibration_runs < 3
        || settings.maximum_recalibrations > 4
    {
        return Err(BenchmarkError(
            "invalid adaptive benchmark settings".to_owned(),
        ));
    }
    Ok(())
}

fn calibrate_batch(
    dataset: &GeneratedDataset,
    implementation: SortImplementation,
    settings: AdaptiveBenchmarkSettings,
) -> Result<usize, BenchmarkError> {
    let mut calibration: Vec<_> = (0..settings.calibration_runs)
        .map(|_| run_once(dataset, implementation))
        .collect();
    calibration.sort_unstable();
    let single_ns = median(&calibration).max(1);
    let buffers = if implementation == SortImplementation::MergeCallerScratch {
        2
    } else {
        1
    };
    let bytes_per_invocation = dataset
        .values
        .len()
        .saturating_mul(std::mem::size_of::<i32>())
        .saturating_mul(buffers);
    let Some(invocations) = calibrated_invocations(
        single_ns,
        settings.target_sample_time_ns,
        bytes_per_invocation,
        settings.maximum_batch_memory_bytes,
    ) else {
        return Err(BenchmarkError(format!(
            "batch memory limit is smaller than one prepared input for {}",
            implementation.id()
        )));
    };
    Ok(invocations)
}

fn calibrated_invocations(
    single_ns: u128,
    target_ns: u128,
    bytes_per_invocation: usize,
    maximum_memory_bytes: usize,
) -> Option<usize> {
    let maximum_by_memory = maximum_memory_bytes / bytes_per_invocation.max(1);
    if maximum_by_memory == 0 {
        return None;
    }
    let single_ns = single_ns.max(1);
    let desired = target_ns.saturating_add(single_ns - 1) / single_ns;
    Some(
        usize::try_from(desired)
            .unwrap_or(usize::MAX)
            .clamp(1, maximum_by_memory),
    )
}

fn rotated_indices(length: usize, round: usize) -> impl Iterator<Item = usize> {
    let start = round % length;
    (0..length).map(move |offset| (start + offset) % length)
}

pub fn recent_windows_are_stable(
    samples: &[u128],
    window: usize,
    tolerance_per_million: u32,
) -> bool {
    if window == 0 || samples.len() < window * 2 {
        return false;
    }
    let split = samples.len() - window;
    let mut previous = samples[split - window..split].to_vec();
    let mut current = samples[split..].to_vec();
    previous.sort_unstable();
    current.sort_unstable();
    let previous_median = median(&previous);
    let current_median = median(&current);
    let scale = previous_median.max(current_median).max(1);
    previous_median
        .abs_diff(current_median)
        .saturating_mul(1_000_000)
        <= scale.saturating_mul(u128::from(tolerance_per_million))
}

pub fn comparability_errors(left: &BenchmarkResult, right: &BenchmarkResult) -> Vec<&'static str> {
    let mut errors = Vec::new();
    if left.context != right.context {
        errors.push("execution contexts differ");
    }
    if left.dataset_digest_sha256 != right.dataset_digest_sha256 {
        errors.push("dataset digests differ");
    }
    match (left.adaptive_settings, right.adaptive_settings) {
        (Some(left), Some(right)) if left != right => {
            errors.push("adaptive benchmark settings differ");
        }
        (None, None) if left.settings != right.settings => {
            errors.push("benchmark settings differ");
        }
        (Some(_), None) | (None, Some(_)) => errors.push("benchmark protocols differ"),
        _ => {}
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
    if summary
        .maximum_ns
        .abs_diff(summary.median_ns)
        .saturating_mul(5)
        > summary.median_ns
        || summary
            .minimum_ns
            .abs_diff(summary.median_ns)
            .saturating_mul(5)
            > summary.median_ns
    {
        warnings.push("an extreme sample differs from median by more than 20%");
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

pub fn position_bias_exceeds(
    samples: &[u128],
    positions: &[usize],
    tolerance_per_million: u32,
) -> bool {
    if samples.len() != positions.len() || samples.is_empty() {
        return true;
    }
    let position_count = positions.iter().max().copied().unwrap_or(0) + 1;
    let mut medians = Vec::with_capacity(position_count);
    for position in 0..position_count {
        let mut group: Vec<_> = samples
            .iter()
            .zip(positions)
            .filter(|(_, candidate)| **candidate == position)
            .map(|(sample, _)| *sample)
            .collect();
        if group.is_empty() {
            return true;
        }
        group.sort_unstable();
        medians.push(median(&group));
    }
    let minimum = *medians.iter().min().unwrap();
    let maximum = *medians.iter().max().unwrap();
    maximum.abs_diff(minimum).saturating_mul(1_000_000)
        > maximum
            .max(1)
            .saturating_mul(u128::from(tolerance_per_million))
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
    PreparedBatch::new(dataset, implementation, 1)
        .measure(dataset)
        .0
}

struct PreparedSort {
    values: Vec<i32>,
    scratch: Vec<i32>,
}

struct PreparedBatch {
    implementation: SortImplementation,
    states: Vec<PreparedSort>,
}

impl PreparedBatch {
    fn new(
        dataset: &GeneratedDataset,
        implementation: SortImplementation,
        invocations: usize,
    ) -> Self {
        let states = (0..invocations)
            .map(|_| PreparedSort {
                values: dataset.values.clone(),
                scratch: if implementation == SortImplementation::MergeCallerScratch {
                    dataset.values.clone()
                } else {
                    Vec::new()
                },
            })
            .collect();
        Self {
            implementation,
            states,
        }
    }

    fn measure(&mut self, dataset: &GeneratedDataset) -> (u128, u128) {
        for state in &mut self.states {
            state.values.clone_from_slice(&dataset.values);
        }
        black_box(&mut self.states);
        let start = Instant::now();
        for state in &mut self.states {
            match self.implementation {
                SortImplementation::MergeAllocating => {
                    merge_sort_by(&mut state.values, i32::cmp);
                }
                SortImplementation::MergeCallerScratch => {
                    merge_sort_by_with_scratch(&mut state.values, &mut state.scratch, i32::cmp)
                        .unwrap();
                }
                SortImplementation::InsertionInPlace => {
                    insertion_sort_by(&mut state.values, i32::cmp);
                }
            }
        }
        let elapsed = start.elapsed().as_nanos();
        black_box(&self.states);
        (elapsed, elapsed / self.states.len() as u128)
    }
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
    use atlas::executions::ExecutionResult;

    use super::{
        AdaptiveBenchmarkSettings, BenchmarkContext, BenchmarkResult, BenchmarkSettings,
        BenchmarkSuite, SampleSummary, SortImplementation, SystemDiagnostics, benchmark_sort,
        calibrated_invocations, comparability_errors, execution_record_from_benchmark,
        parse_cpu_list, position_bias_exceeds, proc_memory_kib, proc_value, process_stat_counter,
        quality_warnings, recent_windows_are_stable, rotated_indices, summarize,
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
    fn adaptive_runs_remain_comparable_with_different_convergence_lengths() {
        let dataset = SORT_BENCHMARK_SPEC
            .generate(&SORT_BENCHMARK_SPEC.cases[0])
            .unwrap();
        let fixed = BenchmarkSettings {
            warmup_runs: 1,
            measured_runs: 3,
        };
        let mut first = benchmark_sort(
            &dataset,
            SortImplementation::MergeAllocating,
            fixed,
            context(),
        )
        .unwrap();
        let mut second = first.clone();
        second.settings.warmup_runs = 99;
        let adaptive = AdaptiveBenchmarkSettings {
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
        first.adaptive_settings = Some(adaptive);
        second.adaptive_settings = Some(adaptive);

        assert!(comparability_errors(&first, &second).is_empty());
    }

    #[test]
    fn quality_checks_flag_dispersion_and_series_drift() {
        let samples = [100, 102, 98, 200, 202, 198];
        let summary = summarize(&samples).unwrap();

        let warnings = quality_warnings(&samples, &summary);

        assert!(warnings.contains(&"median absolute deviation exceeds 5% of median"));
        assert!(warnings.contains(&"sample-series half medians differ by more than 5%"));
        assert!(warnings.contains(&"an extreme sample differs from median by more than 20%"));
    }

    #[test]
    fn adaptive_stability_compares_two_recent_windows() {
        assert!(recent_windows_are_stable(
            &[100, 101, 99, 100, 102, 101, 100, 99],
            4,
            50_000
        ));
        assert!(!recent_windows_are_stable(
            &[200, 205, 195, 200, 100, 102, 98, 100],
            4,
            50_000
        ));
    }

    #[test]
    fn rotated_order_gives_each_implementation_every_position() {
        let rounds: Vec<Vec<_>> = (0..3)
            .map(|round| rotated_indices(3, round).collect())
            .collect();

        assert_eq!(rounds, [vec![0, 1, 2], vec![1, 2, 0], vec![2, 0, 1]]);
    }

    #[test]
    fn parses_proc_key_value_lines_without_platform_calls() {
        let fixture = concat!(
            "Name:\tatlas\n",
            "Cpus_allowed_list:\t0-7\n",
            "voluntary_ctxt_switches:\t42\n",
            "se.nr_migrations : 3\n",
        );

        assert_eq!(proc_value(fixture, "Cpus_allowed_list"), Some("0-7"));
        assert_eq!(proc_value(fixture, "voluntary_ctxt_switches"), Some("42"));
        assert_eq!(proc_value(fixture, "se.nr_migrations"), Some("3"));
        assert_eq!(proc_value(fixture, "missing"), None);
    }

    #[test]
    fn parses_process_memory_kib_from_status() {
        let fixture = "Name:\tatlas\nVmRSS:\t 1234 kB\nVmHWM:\t 5678 kB\n";

        assert_eq!(proc_memory_kib(fixture, "VmRSS"), Some(1_234));
        assert_eq!(proc_memory_kib(fixture, "VmHWM"), Some(5_678));
        assert_eq!(proc_memory_kib(fixture, "VmSize"), None);
    }

    #[test]
    fn parses_page_fault_counters_from_proc_stat() {
        let fixture = "123 (atlas worker) R 1 2 3 4 5 6 7 8 9 10 11";

        assert_eq!(process_stat_counter(fixture, 10), Some(7));
        assert_eq!(process_stat_counter(fixture, 12), Some(9));
    }

    #[test]
    fn calibration_targets_duration_without_exceeding_memory() {
        assert_eq!(
            calibrated_invocations(50_000, 10_000_000, 8_192, 64 * 1024 * 1024),
            Some(200)
        );
        assert_eq!(calibrated_invocations(1, 10_000, 8_192, 16_384), Some(2));
        assert_eq!(calibrated_invocations(1, 10_000, 32_768, 16_384), None);
    }

    #[test]
    fn parses_linux_cpu_lists() {
        assert_eq!(parse_cpu_list("0-3,8,10-11"), [0, 1, 2, 3, 8, 10, 11]);
        assert!(parse_cpu_list("invalid").is_empty());
    }

    #[test]
    fn resolves_only_registered_sorting_implementation_ids() {
        assert_eq!(
            SortImplementation::from_id("sort.merge.rust.slice.v1").unwrap(),
            SortImplementation::MergeAllocating
        );
        assert!(SortImplementation::from_id("sort.unknown").is_err());
    }

    #[test]
    fn detects_execution_position_bias() {
        let samples = [100, 200, 300, 102, 198, 302, 99, 201, 298];
        let positions = [0, 1, 2, 0, 1, 2, 0, 1, 2];

        assert!(position_bias_exceeds(&samples, &positions, 50_000));
        assert!(!position_bias_exceeds(
            &[100, 101, 99, 102, 100, 101],
            &[0, 1, 2, 0, 1, 2],
            50_000
        ));
    }

    #[test]
    fn qualified_benchmark_record_retains_raw_samples_and_protocol() {
        let dataset = SORT_BENCHMARK_SPEC
            .generate(&SORT_BENCHMARK_SPEC.cases[0])
            .unwrap();
        let adaptive = AdaptiveBenchmarkSettings {
            minimum_warmup_rounds: 2,
            maximum_warmup_rounds: 4,
            stability_window: 2,
            required_stable_windows: 1,
            stability_tolerance_per_million: 50_000,
            measured_rounds: 3,
            target_sample_time_ns: 10_000,
            maximum_batch_memory_bytes: 65_536,
            calibration_runs: 2,
            maximum_recalibrations: 1,
        };
        let result = BenchmarkResult {
            implementation_id: SortImplementation::InsertionInPlace.id(),
            dataset_case_id: dataset.case_id,
            dataset_digest_sha256: dataset.content_digest_sha256.clone(),
            element_count: dataset.values.len(),
            seed: dataset.seed,
            settings: BenchmarkSettings {
                warmup_runs: 3,
                measured_runs: 3,
            },
            adaptive_settings: Some(adaptive),
            context: context(),
            correction_checked: true,
            invocations_per_sample: 8,
            warmup_samples_ns: vec![101, 100, 99],
            batch_elapsed_ns: vec![800, 808, 792],
            sample_positions: vec![0, 0, 0],
            samples_ns: vec![100, 101, 99],
            summary: SampleSummary {
                minimum_ns: 99,
                median_ns: 100,
                maximum_ns: 101,
                median_absolute_deviation_ns: 1,
            },
            quality_warnings: Vec::new(),
        };
        let diagnostics = SystemDiagnostics {
            load_average: None,
            allowed_cpus: Some("4".to_owned()),
            voluntary_context_switches: Some(2),
            nonvoluntary_context_switches: Some(3),
            scheduler_migrations: Some(0),
            minor_page_faults: Some(4),
            major_page_faults: Some(0),
            scaling_governors: vec!["performance".to_owned()],
            minimum_observed_frequency_khz: Some(3_000_000),
            maximum_observed_frequency_khz: Some(3_100_000),
            resident_memory_kib: Some(1_000),
            peak_resident_memory_kib: Some(1_200),
        };
        let suite = BenchmarkSuite {
            warmup_rounds: 3,
            recalibrations: 1,
            diagnostics_before: diagnostics.clone(),
            diagnostics_after: diagnostics,
            results: vec![result.clone()],
        };

        let record = execution_record_from_benchmark(
            "test.benchmark.v1",
            "test command".to_owned(),
            "file:test",
            "file:implementation",
            &dataset,
            &suite,
            &result,
        )
        .unwrap();
        let yaml = record.to_yaml().unwrap();
        assert_eq!(
            atlas::executions::ExecutionRecord::from_yaml(&yaml).unwrap(),
            record
        );

        let ExecutionResult::Benchmark(observation) = record.body.result else {
            panic!("expected benchmark observation");
        };
        assert!(observation.qualified);
        assert_eq!(observation.raw_samples.normalized_ns, ["100", "101", "99"]);
        assert_eq!(
            observation.raw_samples.batch_elapsed_ns,
            ["800", "808", "792"]
        );
        assert_eq!(observation.observed["invocations_per_sample"], "8");
        assert_eq!(observation.requested_protocol["measured_rounds"], "3");
        assert_eq!(observation.diagnostics_before["allowed_cpus"], "4");
        assert_eq!(
            observation.metrics["process_peak_resident_memory_kib"],
            "1200"
        );
        assert_eq!(observation.metrics["algorithm_allocations"], "unavailable");
    }

    #[test]
    fn benchmark_record_rejects_quality_warnings() {
        let dataset = SORT_BENCHMARK_SPEC
            .generate(&SORT_BENCHMARK_SPEC.cases[0])
            .unwrap();
        let result = benchmark_sort(
            &dataset,
            SortImplementation::InsertionInPlace,
            BenchmarkSettings {
                warmup_runs: 1,
                measured_runs: 3,
            },
            context(),
        )
        .unwrap();
        let mut warned = result.clone();
        warned.quality_warnings = vec!["synthetic warning"];
        let suite = BenchmarkSuite {
            warmup_rounds: 1,
            recalibrations: 0,
            diagnostics_before: SystemDiagnostics {
                load_average: None,
                allowed_cpus: None,
                voluntary_context_switches: None,
                nonvoluntary_context_switches: None,
                scheduler_migrations: None,
                minor_page_faults: None,
                major_page_faults: None,
                scaling_governors: Vec::new(),
                minimum_observed_frequency_khz: None,
                maximum_observed_frequency_khz: None,
                resident_memory_kib: None,
                peak_resident_memory_kib: None,
            },
            diagnostics_after: SystemDiagnostics {
                load_average: None,
                allowed_cpus: None,
                voluntary_context_switches: None,
                nonvoluntary_context_switches: None,
                scheduler_migrations: None,
                minor_page_faults: None,
                major_page_faults: None,
                scaling_governors: Vec::new(),
                minimum_observed_frequency_khz: None,
                maximum_observed_frequency_khz: None,
                resident_memory_kib: None,
                peak_resident_memory_kib: None,
            },
            results: vec![warned.clone()],
        };

        let error = execution_record_from_benchmark(
            "test.benchmark.v1",
            "test command".to_owned(),
            "file:test",
            "file:implementation",
            &dataset,
            &suite,
            &warned,
        )
        .unwrap_err();

        assert!(error.to_string().contains("refusing unqualified"));
    }
}
