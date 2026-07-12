use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::executions::{
    ExecutionEnvironment, ExecutionParameters, ExecutionRecord, ExecutionResult,
};

pub const EXPERIMENTAL_COMPARISON_FORMAT: &str = "atlas-comparison.experimental.0.2";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonReport {
    pub id: String,
    pub format: String,
    pub execution_ids: Vec<String>,
    pub dataset: ComparisonDataset,
    pub parameters: ExecutionParameters,
    pub environment: ExecutionEnvironment,
    pub context: std::collections::BTreeMap<String, String>,
    pub requested_protocol: std::collections::BTreeMap<String, String>,
    pub results: Vec<ComparisonResult>,
    pub conclusion: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonDataset {
    pub spec_id: String,
    pub case_id: String,
    pub content_digest_sha256: String,
    pub seed: u64,
    pub element_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonResult {
    pub execution_id: String,
    pub implementation_id: String,
    pub median_ns: String,
    pub median_absolute_deviation_ns: String,
    pub process_peak_resident_memory_kib: String,
}

#[derive(Debug)]
pub struct ComparisonError(String);

impl fmt::Display for ComparisonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ComparisonError {}

impl ComparisonReport {
    pub fn from_executions(records: &[ExecutionRecord]) -> Result<Self, ComparisonError> {
        if records.len() < 2 {
            return Err(ComparisonError(
                "comparison requires at least two execution records".to_owned(),
            ));
        }
        let first = benchmark_record(&records[0])?;
        let mut execution_ids = Vec::with_capacity(records.len());
        let mut results = Vec::with_capacity(records.len());
        for record in records {
            let benchmark = benchmark_record(record)?;
            if record.body.dataset != records[0].body.dataset {
                return Err(ComparisonError(
                    "benchmark records use different datasets".to_owned(),
                ));
            }
            if record.body.parameters != records[0].body.parameters {
                return Err(ComparisonError(
                    "benchmark records use different parameters".to_owned(),
                ));
            }
            if record.body.environment != records[0].body.environment {
                return Err(ComparisonError(
                    "benchmark records use different environments".to_owned(),
                ));
            }
            if benchmark.context != first.context {
                return Err(ComparisonError(
                    "benchmark records use different contexts".to_owned(),
                ));
            }
            if benchmark.requested_protocol != first.requested_protocol {
                return Err(ComparisonError(
                    "benchmark records use different requested protocols".to_owned(),
                ));
            }
            if execution_ids.iter().any(|id| id == &record.id) {
                return Err(ComparisonError(format!(
                    "execution {:?} appears more than once",
                    record.id
                )));
            }
            if results.iter().any(|result: &ComparisonResult| {
                result.implementation_id == record.body.implementation_id
            }) {
                return Err(ComparisonError(format!(
                    "implementation {:?} appears more than once",
                    record.body.implementation_id
                )));
            }
            execution_ids.push(record.id.clone());
            results.push(ComparisonResult {
                execution_id: record.id.clone(),
                implementation_id: record.body.implementation_id.clone(),
                median_ns: benchmark.summary.median_ns.clone(),
                median_absolute_deviation_ns: benchmark
                    .summary
                    .median_absolute_deviation_ns
                    .clone(),
                process_peak_resident_memory_kib: benchmark
                    .metrics
                    .get("process_peak_resident_memory_kib")
                    .cloned()
                    .unwrap_or_else(|| "unavailable".to_owned()),
            });
        }
        results.sort_by(|left, right| {
            parse_ns(&left.median_ns)
                .cmp(&parse_ns(&right.median_ns))
                .then_with(|| left.implementation_id.cmp(&right.implementation_id))
        });
        execution_ids.sort();
        let fastest = &results[0];
        let body = ComparisonReportBody {
            format: EXPERIMENTAL_COMPARISON_FORMAT.to_owned(),
            execution_ids: execution_ids.clone(),
            dataset: ComparisonDataset {
                spec_id: records[0].body.dataset.spec_id.clone(),
                case_id: records[0].body.dataset.case_id.clone(),
                content_digest_sha256: records[0].body.dataset.content_digest_sha256.clone(),
                seed: records[0].body.dataset.seed,
                element_count: records[0].body.dataset.element_count,
            },
            parameters: records[0].body.parameters.clone(),
            environment: records[0].body.environment.clone(),
            context: first.context.clone(),
            requested_protocol: first.requested_protocol.clone(),
            results: results.clone(),
            conclusion: format!(
                "Within this exact dataset, context, and requested protocol, {} has the lowest observed median. This is not a general ranking.",
                fastest.implementation_id
            ),
        };
        Ok(Self {
            id: report_id(&body)?,
            format: body.format,
            execution_ids: body.execution_ids,
            dataset: body.dataset,
            parameters: body.parameters,
            environment: body.environment,
            context: body.context,
            requested_protocol: body.requested_protocol,
            results: body.results,
            conclusion: body.conclusion,
        })
    }

    pub fn to_yaml(&self) -> Result<String, ComparisonError> {
        serde_yaml::to_string(self)
            .map_err(|error| ComparisonError(format!("cannot serialize comparison: {error}")))
    }

    pub fn write_yaml(&self, path: &Path) -> Result<(), ComparisonError> {
        let parent = path.parent().ok_or_else(|| {
            ComparisonError(format!(
                "comparison path {:?} has no parent directory",
                path
            ))
        })?;
        fs::create_dir_all(parent).map_err(|error| {
            ComparisonError(format!(
                "cannot create comparison directory {:?}: {error}",
                parent
            ))
        })?;
        fs::write(path, self.to_yaml()?).map_err(|error| {
            ComparisonError(format!("cannot write comparison {:?}: {error}", path))
        })
    }
}

#[derive(Serialize)]
struct ComparisonReportBody {
    format: String,
    execution_ids: Vec<String>,
    dataset: ComparisonDataset,
    parameters: ExecutionParameters,
    environment: ExecutionEnvironment,
    context: std::collections::BTreeMap<String, String>,
    requested_protocol: std::collections::BTreeMap<String, String>,
    results: Vec<ComparisonResult>,
    conclusion: String,
}

fn benchmark_record(
    record: &ExecutionRecord,
) -> Result<&crate::executions::BenchmarkResult, ComparisonError> {
    let ExecutionResult::Benchmark(benchmark) = &record.body.result else {
        return Err(ComparisonError(format!(
            "execution {:?} is not a benchmark observation",
            record.id
        )));
    };
    if !benchmark.qualified || !benchmark.quality_warnings.is_empty() {
        return Err(ComparisonError(format!(
            "execution {:?} is not qualified for comparison",
            record.id
        )));
    }
    if record.body.environment.git_dirty {
        return Err(ComparisonError(format!(
            "execution {:?} was captured from a dirty worktree",
            record.id
        )));
    }
    Ok(benchmark)
}

fn parse_ns(value: &str) -> u128 {
    value.parse().unwrap_or(u128::MAX)
}

fn report_id(body: &ComparisonReportBody) -> Result<String, ComparisonError> {
    let encoded = serde_yaml::to_string(body)
        .map_err(|error| ComparisonError(format!("cannot identify comparison: {error}")))?;
    let digest = Sha256::digest(encoded.as_bytes());
    Ok(format!("comparison.sha256.{}", hex_digest(&digest)))
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use fmt::Write;
        write!(&mut encoded, "{byte:02x}").expect("writing to a String cannot fail");
    }
    encoded
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::executions::{
        BenchmarkRawSamples, BenchmarkResult, BenchmarkSummary, EXPERIMENTAL_EXECUTION_FORMAT,
        ExecutionBody, ExecutionDataset, ExecutionEnvironment, ExecutionMode, ExecutionParameters,
        ExecutionProvenance, ExecutionRecord, ExecutionResult,
    };

    use super::ComparisonReport;

    fn record(implementation_id: &str, median_ns: &str) -> ExecutionRecord {
        ExecutionRecord::from_body(ExecutionBody {
            format: EXPERIMENTAL_EXECUTION_FORMAT.to_owned(),
            recipe_id: "test.benchmark.v1".to_owned(),
            mode: ExecutionMode::Benchmark,
            implementation_id: implementation_id.to_owned(),
            dataset: ExecutionDataset {
                spec_id: "dataset.test".to_owned(),
                case_id: "case.test".to_owned(),
                content_digest_sha256: "a".repeat(64),
                seed: 1,
                element_count: 16,
            },
            parameters: ExecutionParameters {
                value_type: "i32".to_owned(),
                operation: "test".to_owned(),
                build_profile: "release".to_owned(),
            },
            environment: ExecutionEnvironment {
                git_commit: "test".to_owned(),
                git_dirty: false,
                compiler: "rustc test".to_owned(),
                target: "test-target".to_owned(),
            },
            result: ExecutionResult::Benchmark(Box::new(BenchmarkResult {
                qualified: true,
                quality_warnings: Vec::new(),
                context: BTreeMap::from([("cpu_model".to_owned(), "test".to_owned())]),
                requested_protocol: BTreeMap::from([("rounds".to_owned(), "3".to_owned())]),
                observed: BTreeMap::new(),
                raw_samples: BenchmarkRawSamples {
                    warmup_ns: vec!["1".to_owned()],
                    batch_elapsed_ns: vec![median_ns.to_owned()],
                    normalized_ns: vec![median_ns.to_owned()],
                    execution_positions: vec![0],
                },
                summary: BenchmarkSummary {
                    minimum_ns: median_ns.to_owned(),
                    median_ns: median_ns.to_owned(),
                    maximum_ns: median_ns.to_owned(),
                    median_absolute_deviation_ns: "0".to_owned(),
                },
                metrics: BTreeMap::from([(
                    "process_peak_resident_memory_kib".to_owned(),
                    "1024".to_owned(),
                )]),
                diagnostics_before: BTreeMap::new(),
                diagnostics_after: BTreeMap::new(),
                diagnostic_delta: BTreeMap::new(),
            })),
            provenance: ExecutionProvenance {
                command: "test".to_owned(),
                recipe_source: "file:test".to_owned(),
                implementation_source: "file:test".to_owned(),
            },
        })
        .unwrap()
    }

    #[test]
    fn report_is_deterministic_and_bounded_to_its_context() {
        let slower = record("implementation.slower", "200");
        let faster = record("implementation.faster", "100");
        let report = ComparisonReport::from_executions(&[slower.clone(), faster.clone()]).unwrap();

        assert_eq!(
            report,
            ComparisonReport::from_executions(&[slower, faster]).unwrap()
        );
        assert_eq!(report.results[0].implementation_id, "implementation.faster");
        assert_eq!(report.environment.git_commit, "test");
        assert!(report.conclusion.contains("not a general ranking"));
    }

    #[test]
    fn report_rejects_incompatible_protocols() {
        let first = record("implementation.first", "100");
        let mut second = record("implementation.second", "110");
        let ExecutionResult::Benchmark(benchmark) = &mut second.body.result else {
            panic!("test fixture must be a benchmark");
        };
        benchmark
            .requested_protocol
            .insert("rounds".to_owned(), "4".to_owned());
        second = ExecutionRecord::from_body(second.body).unwrap();

        let error = ComparisonReport::from_executions(&[first, second]).unwrap_err();
        assert!(error.to_string().contains("different requested protocols"));
    }

    #[test]
    fn report_rejects_incompatible_environments() {
        let first = record("implementation.first", "100");
        let mut second = record("implementation.second", "110");
        second.body.environment.git_commit = "other".to_owned();
        second = ExecutionRecord::from_body(second.body).unwrap();

        let error = ComparisonReport::from_executions(&[first, second]).unwrap_err();
        assert!(error.to_string().contains("different environments"));
    }

    #[test]
    fn report_rejects_dirty_worktrees() {
        let first = record("implementation.first", "100");
        let mut second = record("implementation.second", "110");
        second.body.environment.git_dirty = true;
        second = ExecutionRecord::from_body(second.body).unwrap();

        let error = ComparisonReport::from_executions(&[first, second]).unwrap_err();
        assert!(error.to_string().contains("dirty worktree"));
    }
}
