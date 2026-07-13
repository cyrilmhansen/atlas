use atlas_mir::{
    JitOptimizationLevel, observe_jit_add_u64, observe_jit_is_sorted_i64,
    observe_jit_partition_even_i64, observe_jit_reverse_i64, summarize_host_code,
};

const OPTIMIZATION_LEVELS: [JitOptimizationLevel; 4] = [
    JitOptimizationLevel::FastGeneration,
    JitOptimizationLevel::RegisterAllocation,
    JitOptimizationLevel::Default,
    JitOptimizationLevel::Full,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("target_arch={}", std::env::consts::ARCH);
    println!("target_os={}", std::env::consts::OS);
    println!(
        "program\toptimization\tresult\tobserved\tprefix\tsuffix\tinstructions\tcalls\tconditional\tunconditional\treturns"
    );

    for optimization in OPTIMIZATION_LEVELS {
        let observation = observe_jit_add_u64(40, 2, optimization)
            .map_err(|error| format!("MIR add observation failed: {error:?}"))?;
        print_shape(
            "add_u64",
            optimization,
            observation.result.to_string(),
            &observation.machine_code,
        )?;
    }
    for optimization in OPTIMIZATION_LEVELS {
        let observation = observe_jit_is_sorted_i64(&[1, 5, 4, 6], optimization)
            .map_err(|error| format!("MIR is_sorted observation failed: {error:?}"))?;
        print_shape(
            "is_sorted_i64",
            optimization,
            format!("{:?}", observation.result),
            &observation.machine_code,
        )?;
    }
    for optimization in OPTIMIZATION_LEVELS {
        let mut values = vec![1, 2, 3, 4, 5];
        let observation = observe_jit_reverse_i64(&mut values, optimization)
            .map_err(|error| format!("MIR reverse observation failed: {error:?}"))?;
        print_shape(
            "reverse_i64",
            optimization,
            format!("{values:?}"),
            &observation.machine_code,
        )?;
    }
    for optimization in OPTIMIZATION_LEVELS {
        let mut values = vec![3, 2, 5, 4, 7, 6];
        let observation = observe_jit_partition_even_i64(&mut values, optimization)
            .map_err(|error| format!("MIR partition observation failed: {error:?}"))?;
        print_shape(
            "partition_even_i64",
            optimization,
            format!("boundary={} values={values:?}", observation.boundary),
            &observation.machine_code,
        )?;
    }
    Ok(())
}

fn print_shape(
    program: &str,
    optimization: JitOptimizationLevel,
    result: String,
    code: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let shape = summarize_host_code(code)?;
    println!(
        "{program}\t{optimization:?}\t{result}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        shape.observed_bytes,
        shape.decoded_prefix_bytes,
        shape.trailing_bytes,
        shape.instructions,
        shape.calls,
        shape.conditional_branches,
        shape.unconditional_branches,
        shape.returns,
    );
    Ok(())
}
