mod support;

use std::collections::BTreeMap;

use atlas::datasets::PARTITION_BENCHMARK_SPEC;
use atlas::executions::digest_i32_values;
use atlas_algorithms::partition::partition_in_place;

use support::{CorrectionRecipe, output_path, write_correction};

const RECIPE: CorrectionRecipe = CorrectionRecipe {
    id: "partition.in_place.alternating.64.correction.v1",
    example: "record_partition_correction",
    implementation_id: "partition.in_place.rust.slice.v1",
    implementation_source: "file:crates/atlas-algorithms/src/partition.rs",
    operation: "partition in place using predicate even",
    default_output: "build/executions/partition-in-place-alternating-64-correction.yaml",
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_path(&RECIPE)?;
    let case = PARTITION_BENCHMARK_SPEC
        .cases
        .iter()
        .find(|case| case.id == "partition.benchmark.alternating.64")
        .ok_or("versioned correction recipe references an unknown dataset case")?;
    let dataset = PARTITION_BENCHMARK_SPEC.generate(case)?;
    let predicate = dataset
        .predicate
        .ok_or("partition recipe requires a predicate")?;
    let mut actual = dataset.values.clone();
    let mut expected = dataset.values.clone();
    expected.sort();
    let boundary = partition_in_place(&mut actual, |value| predicate.matches(*value));
    let separated = actual[..boundary]
        .iter()
        .all(|value| predicate.matches(*value))
        && actual[boundary..]
            .iter()
            .all(|value| !predicate.matches(*value));
    let output_digest = digest_i32_values(&actual);
    let mut sorted_actual = actual;
    sorted_actual.sort();
    let passed = separated && sorted_actual == expected;
    let outputs = BTreeMap::from([
        ("boundary".to_owned(), boundary.to_string()),
        ("sequence_digest_sha256".to_owned(), output_digest),
    ]);

    write_correction(&RECIPE, dataset, passed, outputs, &output)
}
