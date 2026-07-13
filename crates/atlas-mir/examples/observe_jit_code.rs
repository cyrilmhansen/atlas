use std::fmt::Write;

use atlas_mir::{JitOptimizationLevel, observe_jit_add_u64};
use sha2::{Digest, Sha256};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let observation = observe_jit_add_u64(40, 2, JitOptimizationLevel::Default)
        .map_err(|error| format!("MIR JIT observation failed: {error:?}"))?;
    let mut code_hex = String::with_capacity(observation.machine_code.len() * 2);
    for byte in &observation.machine_code {
        write!(&mut code_hex, "{byte:02x}")?;
    }
    let digest = Sha256::digest(&observation.machine_code);
    let mut digest_hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(&mut digest_hex, "{byte:02x}")?;
    }

    println!("target_arch={}", std::env::consts::ARCH);
    println!("target_os={}", std::env::consts::OS);
    println!("optimization={:?}", observation.optimization);
    println!("result={}", observation.result);
    println!("machine_code_bytes={}", observation.machine_code.len());
    println!("machine_code_sha256={digest_hex}");
    println!("machine_code_hex={code_hex}");
    Ok(())
}
