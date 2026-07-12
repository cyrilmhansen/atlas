use atlas::{
    datasets::{PARTITION_DATASET_SPEC, SORT_DATASET_SPEC},
    traces::{trace_merge_sort, trace_partition_in_place},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sort_dataset = SORT_DATASET_SPEC.generate(&SORT_DATASET_SPEC.cases[4])?;
    let partition_dataset = PARTITION_DATASET_SPEC.generate(&PARTITION_DATASET_SPEC.cases[3])?;

    for trace in [
        trace_merge_sort(&sort_dataset),
        trace_partition_in_place(&partition_dataset),
    ] {
        println!(
            "{} / {} / {}",
            trace.algorithm_id, trace.implementation_id, trace.dataset_case_id
        );
        println!("  dataset sha256: {}", trace.dataset_digest_sha256);
        println!("  result: {:?}", trace.result);
        for (index, step) in trace.steps.iter().enumerate() {
            println!("  {index:03}: {} -> {:?}", step.ast_node_id, step.event);
        }
    }
    Ok(())
}
