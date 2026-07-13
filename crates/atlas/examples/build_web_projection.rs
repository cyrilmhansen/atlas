use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use atlas::index::summarize_registry;
use atlas::registry::load_registry;
use atlas::web_projection::{WebBuildEnvironment, to_json};

fn main() -> ExitCode {
    let mut arguments = env::args_os().skip(1);
    let Some(output_path) = arguments.next().map(PathBuf::from) else {
        eprintln!(
            "usage: build_web_projection OUTPUT_PATH SOURCE_COMMIT RUSTC_VERSION WASM_BINDGEN_VERSION"
        );
        return ExitCode::from(2);
    };
    let Some(source_commit) = arguments.next().and_then(|value| value.into_string().ok()) else {
        eprintln!("build_web_projection requires a UTF-8 source commit");
        return ExitCode::from(2);
    };
    let Some(rustc_version) = arguments.next().and_then(|value| value.into_string().ok()) else {
        eprintln!("build_web_projection requires a UTF-8 rustc version");
        return ExitCode::from(2);
    };
    let Some(wasm_bindgen_version) = arguments.next().and_then(|value| value.into_string().ok())
    else {
        eprintln!("build_web_projection requires a UTF-8 wasm-bindgen version");
        return ExitCode::from(2);
    };
    if arguments.next().is_some() {
        eprintln!("build_web_projection accepts exactly two arguments");
        return ExitCode::from(2);
    }
    if !(7..=64).contains(&source_commit.len())
        || !source_commit.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        eprintln!("source commit must contain 7 to 64 hexadecimal characters");
        return ExitCode::from(2);
    }

    match build(
        &output_path,
        &source_commit,
        &rustc_version,
        &wasm_bindgen_version,
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("cannot build {}: {error}", output_path.display());
            ExitCode::FAILURE
        }
    }
}

fn build(
    output_path: &Path,
    source_commit: &str,
    rustc_version: &str,
    wasm_bindgen_version: &str,
) -> Result<(), String> {
    let registry =
        load_registry(Path::new("registry/atlas.yaml")).map_err(|error| error.to_string())?;
    let summary = summarize_registry(&registry).map_err(|error| error.to_string())?;
    let build = WebBuildEnvironment {
        rustc: rustc_version,
        wasm_bindgen: wasm_bindgen_version,
        target: "wasm32-unknown-unknown",
        profile: "release",
    };
    let json =
        to_json(&registry, &summary, source_commit, build).map_err(|error| error.to_string())?;
    let parent = output_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .ok_or_else(|| "output path must have a parent directory".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(output_path, format!("{json}\n")).map_err(|error| error.to_string())?;
    println!(
        "Projected {} problems, {} algorithms and {} implementations with digest {}",
        registry.problems.len(),
        registry.algorithms.len(),
        registry.implementations.len(),
        summary.digest
    );
    Ok(())
}
