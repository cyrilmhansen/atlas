use atlas::datasets::{
    PARTITION_BENCHMARK_SPEC, PARTITION_DATASET_SPEC, SORT_BENCHMARK_SPEC, SORT_DATASET_SPEC,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for spec in [
        &SORT_DATASET_SPEC,
        &PARTITION_DATASET_SPEC,
        &SORT_BENCHMARK_SPEC,
        &PARTITION_BENCHMARK_SPEC,
    ] {
        println!("{} -> {}", spec.id, spec.problem_id);
        for dataset in spec.generate_all()? {
            println!(
                "  {} class={:?} seed={} length={} sha256={}",
                dataset.case_id,
                dataset.class,
                dataset.seed,
                dataset.values.len(),
                dataset.content_digest_sha256
            );
        }
    }
    Ok(())
}
