use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let workspace = manifest.join("../..");
    let mir = workspace.join("vendor/mir");
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("output directory"));
    let cc = env::var("CC").unwrap_or_else(|_| "cc".to_owned());
    let ar = env::var("AR").unwrap_or_else(|_| "ar".to_owned());

    let mir_object = out.join("mir.o");
    run(
        Command::new(&cc)
            .arg("-std=gnu11")
            .arg("-O2")
            .arg("-fPIC")
            .arg("-I")
            .arg(&mir)
            .arg("-c")
            .arg(mir.join("mir.c"))
            .arg("-o")
            .arg(&mir_object),
        "compile MIR interpreter core",
    );
    let shim_object = out.join("atlas_mir_shim.o");
    run(
        Command::new(&cc)
            .arg("-std=c11")
            .arg("-I")
            .arg(&mir)
            .arg("-c")
            .arg(manifest.join("src/mir_shim.c"))
            .arg("-o")
            .arg(&shim_object),
        "compile Atlas MIR shim",
    );
    run(
        Command::new(&ar)
            .arg("crus")
            .arg(out.join("libmir.a"))
            .arg(mir_object),
        "archive MIR interpreter core",
    );
    run(
        Command::new(&ar)
            .arg("crus")
            .arg(out.join("libatlas_mir_shim.a"))
            .arg(shim_object),
        "archive Atlas MIR shim",
    );

    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=atlas_mir_shim");
    println!("cargo:rustc-link-lib=static=mir");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=m");
    for path in [
        manifest.join("src/mir_shim.c"),
        mir.join("mir.c"),
        mir.join("mir.h"),
        mir.join("mir-interp.c"),
    ] {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

fn run(command: &mut Command, description: &str) {
    let status = command
        .status()
        .unwrap_or_else(|error| panic!("{description}: could not start command: {error}"));
    assert!(
        status.success(),
        "{description}: command failed with {status}"
    );
}
