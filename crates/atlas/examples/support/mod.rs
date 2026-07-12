use std::collections::BTreeMap;
use std::env;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use atlas::datasets::GeneratedDataset;
use atlas::executions::{
    CorrectionResult, EXPERIMENTAL_EXECUTION_FORMAT, ExecutionBody, ExecutionDataset,
    ExecutionEnvironment, ExecutionMode, ExecutionParameters, ExecutionProvenance, ExecutionRecord,
    ExecutionResult,
};
use atlas::registry::load_registry;

pub struct CorrectionRecipe {
    pub id: &'static str,
    pub example: &'static str,
    pub implementation_id: &'static str,
    pub implementation_source: &'static str,
    pub operation: &'static str,
    pub default_output: &'static str,
}

pub fn output_path(recipe: &CorrectionRecipe) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let output = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(recipe.default_output));
    if arguments.next().is_some() {
        return Err(format!("{} accepts at most one output path", recipe.example).into());
    }
    if !output.starts_with(Path::new("build/executions"))
        || output
            .components()
            .any(|component| component == Component::ParentDir)
    {
        return Err("execution output must remain under build/executions".into());
    }
    Ok(output)
}

pub fn write_correction(
    recipe: &CorrectionRecipe,
    dataset: GeneratedDataset,
    passed: bool,
    outputs: BTreeMap<String, String>,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let registry = load_registry(Path::new("registry/atlas.yaml"))
        .map_err(|error| format!("cannot load registry/atlas.yaml: {error}"))?;
    if !registry
        .implementations
        .iter()
        .any(|implementation| implementation.id == recipe.implementation_id)
    {
        return Err(format!(
            "implementation {:?} is not registered",
            recipe.implementation_id
        )
        .into());
    }

    let record = ExecutionRecord::from_body(ExecutionBody {
        format: EXPERIMENTAL_EXECUTION_FORMAT.to_owned(),
        recipe_id: recipe.id.to_owned(),
        mode: ExecutionMode::Correction,
        implementation_id: recipe.implementation_id.to_owned(),
        dataset: ExecutionDataset {
            spec_id: dataset.spec_id.to_owned(),
            case_id: dataset.case_id.to_owned(),
            content_digest_sha256: dataset.content_digest_sha256,
            seed: dataset.seed,
            element_count: dataset.values.len(),
        },
        parameters: ExecutionParameters {
            value_type: "i32".to_owned(),
            operation: recipe.operation.to_owned(),
            build_profile: if cfg!(debug_assertions) {
                "debug".to_owned()
            } else {
                "release".to_owned()
            },
        },
        environment: capture_environment()?,
        result: ExecutionResult::Correction(CorrectionResult { passed, outputs }),
        provenance: ExecutionProvenance {
            command: format!(
                "cargo run -p atlas --example {} -- {}",
                recipe.example,
                output_path.display()
            ),
            recipe_source: format!("file:crates/atlas/examples/{}.rs", recipe.example),
            implementation_source: recipe.implementation_source.to_owned(),
        },
    })?;
    if !passed {
        return Err("correction recipe failed; no execution was written".into());
    }
    record.write_yaml(output_path)?;
    println!("Wrote {} to {}", record.id, output_path.display());
    Ok(())
}

fn capture_environment() -> Result<ExecutionEnvironment, Box<dyn std::error::Error>> {
    let rustc = command_output("rustc", &["-vV"])?;
    let compiler = rustc
        .lines()
        .next()
        .unwrap_or("rustc unavailable")
        .to_owned();
    let target = rustc
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .unwrap_or("unavailable")
        .to_owned();
    Ok(ExecutionEnvironment {
        git_commit: command_output("git", &["rev-parse", "HEAD"])?,
        git_dirty: !command_output("git", &["status", "--porcelain"])?.is_empty(),
        compiler,
        target,
    })
}

fn command_output(program: &str, arguments: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(program).args(arguments).output()?;
    if !output.status.success() {
        return Err(format!(
            "{program} exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}
