mod support;

use std::collections::BTreeMap;

use atlas::datasets::SORT_BENCHMARK_SPEC;
use atlas::executions::digest_i32_values;
use atlas_algorithms::insertion_sort::insertion_sort_by;

use support::{CorrectionRecipe, output_path, write_correction};

const RECIPE: CorrectionRecipe = CorrectionRecipe {
    id: "sort.insertion.uniform.64.correction.v1",
    example: "record_sort_correction",
    implementation_id: "sort.insertion.rust.slice.v1",
    implementation_source: "file:crates/atlas-algorithms/src/insertion_sort.rs",
    operation: "sort using i32::cmp",
    default_output: "build/executions/sort-insertion-uniform-64-correction.yaml",
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_path(&RECIPE)?;
    let case = SORT_BENCHMARK_SPEC
        .cases
        .iter()
        .find(|case| case.id == "sort.benchmark.uniform.64")
        .ok_or("versioned correction recipe references an unknown dataset case")?;
    let dataset = SORT_BENCHMARK_SPEC.generate(case)?;
    let mut actual = dataset.values.clone();
    let mut expected = dataset.values.clone();
    insertion_sort_by(&mut actual, i32::cmp);
    expected.sort();
    let outputs = BTreeMap::from([(
        "sequence_digest_sha256".to_owned(),
        digest_i32_values(&actual),
    )]);

    write_correction(&RECIPE, dataset, actual == expected, outputs, &output)
}
