use std::fmt::Write;

use atlas_mir::{JitOptimizationLevel, observe_jit_add_u64, observe_jit_is_sorted_i64};
use sha2::{Digest, Sha256};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let add = observe_jit_add_u64(40, 2, JitOptimizationLevel::Default)
        .map_err(|error| format!("MIR JIT observation failed: {error:?}"))?;
    let is_sorted = observe_jit_is_sorted_i64(&[1, 5, 4, 6], JitOptimizationLevel::Default)
        .map_err(|error| format!("MIR guest JIT observation failed: {error:?}"))?;

    println!("target_arch={}", std::env::consts::ARCH);
    println!("target_os={}", std::env::consts::OS);
    print_code(
        "add_u64",
        add.optimization,
        &format!("{}", add.result),
        &add.machine_code,
    )?;
    print_code(
        "is_sorted_i64",
        is_sorted.optimization,
        &format!("{:?}", is_sorted.result),
        &is_sorted.machine_code,
    )?;
    Ok(())
}

fn print_code(
    name: &str,
    optimization: JitOptimizationLevel,
    result: &str,
    code: &[u8],
) -> Result<(), std::fmt::Error> {
    let mut code_hex = String::with_capacity(code.len() * 2);
    for byte in code {
        write!(&mut code_hex, "{byte:02x}")?;
    }
    let digest = Sha256::digest(code);
    let mut digest_hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(&mut digest_hex, "{byte:02x}")?;
    }

    println!("{name}.optimization={optimization:?}");
    println!("{name}.result={result}");
    println!("{name}.machine_code_bytes={}", code.len());
    println!("{name}.machine_code_sha256={digest_hex}");
    println!("{name}.machine_code_hex={code_hex}");
    Ok(())
}
