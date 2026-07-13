use std::fmt::Write;

use atlas_mir::{
    JitOptimizationLevel, disassemble_host_code, observe_jit_add_u64, observe_jit_is_sorted_i64,
};
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
) -> Result<(), Box<dyn std::error::Error>> {
    let code_hex = encode_hex(code)?;
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
    let disassembly = disassemble_host_code(code)?;
    println!("{name}.disassembly_arch={}", disassembly.architecture);
    for instruction in &disassembly.instructions {
        println!(
            "{name}.instruction=+0x{:04x}\t{:<20}\t{}\t{}",
            instruction.offset,
            encode_hex(&instruction.bytes)?,
            instruction.mnemonic,
            instruction.operands,
        );
    }
    println!("{name}.decoded_bytes={}", disassembly.decoded_bytes);
    println!("{name}.termination={:?}", disassembly.termination);
    println!(
        "{name}.trailing_bytes_hex={}",
        encode_hex(&disassembly.trailing_bytes)?
    );
    Ok(())
}

fn encode_hex(bytes: &[u8]) -> Result<String, std::fmt::Error> {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}")?;
    }
    Ok(encoded)
}
