use std::fmt::{self, Write};

use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatasetClass {
    Typical,
    Boundary,
    Degenerate,
    Adversarial,
    Regression,
}

impl DatasetClass {
    fn tag(self) -> &'static str {
        match self {
            Self::Typical => "typical",
            Self::Boundary => "boundary",
            Self::Degenerate => "degenerate",
            Self::Adversarial => "adversarial",
            Self::Regression => "regression",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntGenerator {
    Literal(&'static [i32]),
    SeededUniform {
        length: usize,
        min: i32,
        max: i32,
    },
    Ascending {
        length: usize,
    },
    Descending {
        length: usize,
    },
    Repeated {
        length: usize,
        value: i32,
    },
    Alternating {
        length: usize,
        first: i32,
        second: i32,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntPredicate {
    Even,
    LessThan(i32),
    Always,
    Never,
}

impl IntPredicate {
    pub fn matches(self, value: i32) -> bool {
        match self {
            Self::Even => value % 2 == 0,
            Self::LessThan(limit) => value < limit,
            Self::Always => true,
            Self::Never => false,
        }
    }

    fn canonical(self) -> String {
        match self {
            Self::Even => "even".to_owned(),
            Self::LessThan(limit) => format!("less_than:{limit}"),
            Self::Always => "always".to_owned(),
            Self::Never => "never".to_owned(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DatasetCaseSpec {
    pub id: &'static str,
    pub class: DatasetClass,
    pub seed: u64,
    pub generator: IntGenerator,
    pub predicate: Option<IntPredicate>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DatasetSpec {
    pub id: &'static str,
    pub problem_id: &'static str,
    pub cases: &'static [DatasetCaseSpec],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedDataset {
    pub spec_id: &'static str,
    pub case_id: &'static str,
    pub problem_id: &'static str,
    pub class: DatasetClass,
    pub seed: u64,
    pub values: Vec<i32>,
    pub predicate: Option<IntPredicate>,
    pub content_digest_sha256: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct GenerationError {
    pub case_id: &'static str,
    pub message: String,
}

impl fmt::Display for GenerationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "dataset case {:?}: {}",
            self.case_id, self.message
        )
    }
}

impl std::error::Error for GenerationError {}

impl DatasetSpec {
    pub fn generate(
        &'static self,
        case: &'static DatasetCaseSpec,
    ) -> Result<GeneratedDataset, GenerationError> {
        let values = generate_values(case)?;
        let digest = content_digest(self, case, &values);
        Ok(GeneratedDataset {
            spec_id: self.id,
            case_id: case.id,
            problem_id: self.problem_id,
            class: case.class,
            seed: case.seed,
            values,
            predicate: case.predicate,
            content_digest_sha256: digest,
        })
    }

    pub fn generate_all(&'static self) -> Result<Vec<GeneratedDataset>, GenerationError> {
        self.cases.iter().map(|case| self.generate(case)).collect()
    }
}

fn generate_values(case: &DatasetCaseSpec) -> Result<Vec<i32>, GenerationError> {
    let values = match case.generator {
        IntGenerator::Literal(values) => values.to_vec(),
        IntGenerator::SeededUniform { length, min, max } => {
            if min > max {
                return Err(GenerationError {
                    case_id: case.id,
                    message: format!("uniform minimum {min} exceeds maximum {max}"),
                });
            }
            let width = i64::from(max) - i64::from(min) + 1;
            let mut state = case.seed;
            (0..length)
                .map(|_| {
                    let random = splitmix64(&mut state);
                    (i64::from(min) + (random % width as u64) as i64) as i32
                })
                .collect()
        }
        IntGenerator::Ascending { length } => (0..length).map(index_as_i32).collect(),
        IntGenerator::Descending { length } => (0..length).rev().map(index_as_i32).collect(),
        IntGenerator::Repeated { length, value } => vec![value; length],
        IntGenerator::Alternating {
            length,
            first,
            second,
        } => (0..length)
            .map(|index| if index % 2 == 0 { first } else { second })
            .collect(),
    };
    Ok(values)
}

fn index_as_i32(index: usize) -> i32 {
    i32::try_from(index).expect("MVP 2 static dataset lengths fit in i32")
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn content_digest(spec: &DatasetSpec, case: &DatasetCaseSpec, values: &[i32]) -> String {
    let mut hash = Sha256::new();
    hash_field(&mut hash, spec.id.as_bytes());
    hash_field(&mut hash, case.id.as_bytes());
    hash_field(&mut hash, spec.problem_id.as_bytes());
    hash_field(&mut hash, case.class.tag().as_bytes());
    hash_field(&mut hash, &case.seed.to_le_bytes());
    match case.predicate {
        Some(predicate) => hash_field(&mut hash, predicate.canonical().as_bytes()),
        None => hash_field(&mut hash, b"none"),
    }
    hash_field(&mut hash, &values.len().to_le_bytes());
    for value in values {
        hash_field(&mut hash, &value.to_le_bytes());
    }
    let bytes = hash.finalize();
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}").expect("writing to a String cannot fail");
    }
    encoded
}

fn hash_field(hash: &mut Sha256, value: &[u8]) {
    hash.update(value.len().to_le_bytes());
    hash.update(value);
}

const SORT_CASES: &[DatasetCaseSpec] = &[
    DatasetCaseSpec {
        id: "sort.typical.seeded_uniform",
        class: DatasetClass::Typical,
        seed: 1,
        generator: IntGenerator::SeededUniform {
            length: 16,
            min: -20,
            max: 20,
        },
        predicate: None,
    },
    DatasetCaseSpec {
        id: "sort.boundary.empty",
        class: DatasetClass::Boundary,
        seed: 0,
        generator: IntGenerator::Literal(&[]),
        predicate: None,
    },
    DatasetCaseSpec {
        id: "sort.degenerate.equal",
        class: DatasetClass::Degenerate,
        seed: 0,
        generator: IntGenerator::Repeated {
            length: 12,
            value: 7,
        },
        predicate: None,
    },
    DatasetCaseSpec {
        id: "sort.adversarial.descending",
        class: DatasetClass::Adversarial,
        seed: 0,
        generator: IntGenerator::Descending { length: 32 },
        predicate: None,
    },
    DatasetCaseSpec {
        id: "sort.regression.duplicates",
        class: DatasetClass::Regression,
        seed: 0,
        generator: IntGenerator::Literal(&[5, -1, 5, 3, 0, -8, 3]),
        predicate: None,
    },
];

const PARTITION_CASES: &[DatasetCaseSpec] = &[
    DatasetCaseSpec {
        id: "partition.typical.mixed_sign",
        class: DatasetClass::Typical,
        seed: 0,
        generator: IntGenerator::Literal(&[-3, 4, -1, 0, 8, -7]),
        predicate: Some(IntPredicate::LessThan(0)),
    },
    DatasetCaseSpec {
        id: "partition.boundary.empty",
        class: DatasetClass::Boundary,
        seed: 0,
        generator: IntGenerator::Literal(&[]),
        predicate: Some(IntPredicate::Even),
    },
    DatasetCaseSpec {
        id: "partition.degenerate.all_matching",
        class: DatasetClass::Degenerate,
        seed: 0,
        generator: IntGenerator::Ascending { length: 12 },
        predicate: Some(IntPredicate::Always),
    },
    DatasetCaseSpec {
        id: "partition.adversarial.alternating",
        class: DatasetClass::Adversarial,
        seed: 0,
        generator: IntGenerator::Alternating {
            length: 32,
            first: 0,
            second: 1,
        },
        predicate: Some(IntPredicate::Even),
    },
    DatasetCaseSpec {
        id: "partition.regression.none_matching",
        class: DatasetClass::Regression,
        seed: 0,
        generator: IntGenerator::Literal(&[1, 3, 5, 7]),
        predicate: Some(IntPredicate::Even),
    },
];

pub static SORT_DATASET_SPEC: DatasetSpec = DatasetSpec {
    id: "dataset.sequence.sort.m2.v0",
    problem_id: "sequence.sort",
    cases: SORT_CASES,
};

pub static PARTITION_DATASET_SPEC: DatasetSpec = DatasetSpec {
    id: "dataset.sequence.partition.m2.v0",
    problem_id: "sequence.partition",
    cases: PARTITION_CASES,
};

const SORT_BENCHMARK_CASES: &[DatasetCaseSpec] = &[DatasetCaseSpec {
    id: "sort.benchmark.seeded_uniform.2048",
    class: DatasetClass::Typical,
    seed: 20_260_712,
    generator: IntGenerator::SeededUniform {
        length: 2_048,
        min: -10_000,
        max: 10_000,
    },
    predicate: None,
}];

pub static SORT_BENCHMARK_SPEC: DatasetSpec = DatasetSpec {
    id: "dataset.sequence.sort.benchmark.m2.v0",
    problem_id: "sequence.sort",
    cases: SORT_BENCHMARK_CASES,
};

#[cfg(test)]
mod tests {
    use atlas_algorithms::{insertion_sort::insertion_sort_by, partition::partition_in_place};

    use super::{
        DatasetCaseSpec, DatasetClass, IntGenerator, PARTITION_DATASET_SPEC, SORT_DATASET_SPEC,
    };

    #[test]
    fn specs_cover_all_required_case_classes() {
        let expected = [
            DatasetClass::Typical,
            DatasetClass::Boundary,
            DatasetClass::Degenerate,
            DatasetClass::Adversarial,
            DatasetClass::Regression,
        ];
        for spec in [&SORT_DATASET_SPEC, &PARTITION_DATASET_SPEC] {
            assert_eq!(
                spec.cases.iter().map(|case| case.class).collect::<Vec<_>>(),
                expected
            );
        }
    }

    #[test]
    fn repeated_generation_is_identical() {
        let first = SORT_DATASET_SPEC.generate_all().unwrap();
        let second = SORT_DATASET_SPEC.generate_all().unwrap();

        assert_eq!(first, second);
        assert!(
            first
                .iter()
                .all(|dataset| dataset.content_digest_sha256.len() == 64)
        );
    }

    #[test]
    fn a_seed_change_changes_values_and_digest() {
        static CHANGED_SEED: DatasetCaseSpec = DatasetCaseSpec {
            id: "sort.typical.seeded_uniform",
            class: DatasetClass::Typical,
            seed: 2,
            generator: IntGenerator::SeededUniform {
                length: 16,
                min: -20,
                max: 20,
            },
            predicate: None,
        };
        let original = SORT_DATASET_SPEC
            .generate(&SORT_DATASET_SPEC.cases[0])
            .unwrap();
        let changed = SORT_DATASET_SPEC.generate(&CHANGED_SEED).unwrap();

        assert_ne!(original.values, changed.values);
        assert_ne!(
            original.content_digest_sha256,
            changed.content_digest_sha256
        );
    }

    #[test]
    fn every_sort_case_satisfies_the_sort_contract() {
        for mut dataset in SORT_DATASET_SPEC.generate_all().unwrap() {
            let mut expected = dataset.values.clone();
            expected.sort();
            insertion_sort_by(&mut dataset.values, i32::cmp);
            assert_eq!(dataset.values, expected, "case {}", dataset.case_id);
        }
    }

    #[test]
    fn every_partition_case_satisfies_the_partition_contract() {
        for mut dataset in PARTITION_DATASET_SPEC.generate_all().unwrap() {
            let predicate = dataset.predicate.expect("partition cases need a predicate");
            let mut expected = dataset.values.clone();
            expected.sort();
            let boundary =
                partition_in_place(&mut dataset.values, |value| predicate.matches(*value));

            assert!(
                dataset.values[..boundary]
                    .iter()
                    .all(|value| predicate.matches(*value))
            );
            assert!(
                dataset.values[boundary..]
                    .iter()
                    .all(|value| !predicate.matches(*value))
            );
            dataset.values.sort();
            assert_eq!(dataset.values, expected, "case {}", dataset.case_id);
        }
    }
}
