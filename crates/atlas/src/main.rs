use std::env;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use atlas::comparisons::ComparisonReport;
use atlas::composition::{
    ImplementationConstraint, apply_implementation_constraint,
    cleanup_minimize_declared_allocations, cleanup_minimize_declared_expected_time,
    find_minimize_declared_allocations, render as render_composition,
    render_expected_time_rust_orchestration, render_find_rust_orchestration,
    render_rust_orchestration,
};
use atlas::executions::{ExecutionMode, ExecutionRecord};
use atlas::index::rebuild_database;
use atlas::registry::{
    Algorithm, Claim, Effects, Implementation, Problem, Registry, load_registry,
};

const DEFAULT_REGISTRY: &str = "registry/atlas.yaml";
const DEFAULT_DATABASE: &str = "build/atlas.sqlite3";
const EXECUTION_DIRECTORY: &str = "build/executions";
const REPORT_DIRECTORY: &str = "build/reports";

fn main() -> ExitCode {
    let mut arguments = env::args_os().skip(1);
    let Some(command) = arguments.next() else {
        print_usage();
        return ExitCode::from(2);
    };

    match command.to_str() {
        Some("validate") => validate_command(arguments),
        Some("list") => list_command(arguments),
        Some("show") => show_command(arguments),
        Some("search") => search_command(arguments),
        Some("explain") => explain_command(arguments),
        Some("qualify") => qualify_command(arguments),
        Some("replay") => replay_command(arguments),
        Some("compare") => compare_command(arguments),
        Some("compose") => compose_command(arguments),
        Some("index") => index_command(arguments),
        _ => {
            eprintln!("unknown command {:?}", command);
            print_usage();
            ExitCode::from(2)
        }
    }
}

fn compose_command(mut arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let Some(scenario) = arguments.next() else {
        eprintln!("compose requires the experimental scenario cleanup or find");
        print_usage();
        return ExitCode::from(2);
    };
    let Some(scenario) = scenario.to_str() else {
        eprintln!("composition scenario must be valid UTF-8");
        return ExitCode::from(2);
    };
    if !matches!(scenario, "cleanup" | "find") {
        eprintln!("unknown composition scenario {scenario:?}; expected cleanup or find");
        return ExitCode::from(2);
    }
    let mut render_rust = false;
    let mut expected_time = false;
    let mut force = None;
    let mut forbid = None;
    while let Some(option) = arguments.next() {
        match option.to_str() {
            Some("--rust") if !render_rust => render_rust = true,
            Some("--goal") if !expected_time => match arguments.next().as_deref() {
                Some(value) if value == "expected-time" && scenario == "cleanup" => {
                    expected_time = true
                }
                Some(value) if value == "expected-time" => {
                    eprintln!("expected-time is currently supported only for cleanup");
                    return ExitCode::from(2);
                }
                Some(value) => {
                    eprintln!("unknown compose goal {:?}; expected expected-time", value);
                    return ExitCode::from(2);
                }
                None => {
                    eprintln!("--goal requires expected-time");
                    return ExitCode::from(2);
                }
            },
            Some("--force") if force.is_none() && forbid.is_none() => {
                let Some(id) = arguments.next().and_then(|id| id.into_string().ok()) else {
                    eprintln!("--force requires an implementation ID in valid UTF-8");
                    return ExitCode::from(2);
                };
                force = Some(id);
            }
            Some("--forbid") if forbid.is_none() && force.is_none() => {
                let Some(id) = arguments.next().and_then(|id| id.into_string().ok()) else {
                    eprintln!("--forbid requires an implementation ID in valid UTF-8");
                    return ExitCode::from(2);
                };
                forbid = Some(id);
            }
            Some(option) => {
                eprintln!(
                    "unknown compose option {option:?}; expected --goal expected-time, --force ID, --forbid ID, or --rust"
                );
                return ExitCode::from(2);
            }
            None => {
                eprintln!("compose options must be valid UTF-8");
                return ExitCode::from(2);
            }
        }
    }
    let constraint = match (force.as_deref(), forbid.as_deref()) {
        (Some(id), None) => Some(ImplementationConstraint::Force(id)),
        (None, Some(id)) => Some(ImplementationConstraint::Forbid(id)),
        (None, None) => None,
        (Some(_), Some(_)) => unreachable!("parser keeps force and forbid exclusive"),
    };
    if render_rust && constraint.is_some() {
        eprintln!(
            "--rust is unavailable with --force or --forbid until that exact source is verified"
        );
        return ExitCode::from(2);
    }
    if render_rust {
        match (scenario, expected_time) {
            ("cleanup", true) => print!("{}", render_expected_time_rust_orchestration()),
            ("cleanup", false) => print!("{}", render_rust_orchestration()),
            ("find", false) => print!("{}", render_find_rust_orchestration()),
            ("find", true) => unreachable!("expected-time is rejected for find"),
            _ => unreachable!("scenario is validated before rendering"),
        }
    } else {
        let composition = match (scenario, expected_time) {
            ("cleanup", true) => cleanup_minimize_declared_expected_time(),
            ("cleanup", false) => cleanup_minimize_declared_allocations(),
            ("find", false) => find_minimize_declared_allocations(),
            ("find", true) => unreachable!("expected-time is rejected for find"),
            _ => unreachable!("scenario is validated before rendering"),
        };
        let composition = match constraint {
            Some(constraint) => match apply_implementation_constraint(composition, constraint) {
                Ok(composition) => composition,
                Err(error) => {
                    eprintln!("cannot compose {scenario:?}: {error}");
                    return ExitCode::from(2);
                }
            },
            None => composition,
        };
        print!("{}", render_composition(&composition));
    }
    ExitCode::SUCCESS
}

fn index_command(mut arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let database_path = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_DATABASE));
    if arguments.next().is_some() {
        eprintln!("index accepts at most one database path");
        print_usage();
        return ExitCode::from(2);
    }

    let registry = match load_registry(Path::new(DEFAULT_REGISTRY)) {
        Ok(registry) => registry,
        Err(error) => {
            eprintln!("{DEFAULT_REGISTRY}: {error}");
            return ExitCode::FAILURE;
        }
    };
    match rebuild_database(&registry, &database_path) {
        Ok(summary) => {
            println!(
                "Indexed {} entities, {} relations, {} claims into {}",
                summary.entities,
                summary.relations,
                summary.claims,
                database_path.display()
            );
            println!("Logical SHA-256: {}", summary.digest);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{}: {error}", database_path.display());
            ExitCode::FAILURE
        }
    }
}

fn compare_command(arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let mut ids = Vec::new();
    for argument in arguments {
        let Some(id) = argument.to_str() else {
            eprintln!("execution IDs must be valid UTF-8");
            return ExitCode::from(2);
        };
        ids.push(id.to_owned());
    }
    if ids.len() < 2 {
        eprintln!("compare requires at least two execution IDs");
        print_usage();
        return ExitCode::from(2);
    }
    let mut records = Vec::with_capacity(ids.len());
    for id in &ids {
        match find_execution(id) {
            Ok((_, record)) => records.push(record),
            Err(message) => {
                eprintln!("{message}");
                return ExitCode::FAILURE;
            }
        }
    }
    let report = match ComparisonReport::from_executions(&records) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("cannot compare executions: {error}");
            return ExitCode::FAILURE;
        }
    };
    let path = Path::new(REPORT_DIRECTORY).join(format!("{}.yaml", report.id));
    match report.write_yaml(&path) {
        Ok(()) => {
            println!("Wrote {} to {}", report.id, path.display());
            println!("{}", report.conclusion);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{}: {error}", path.display());
            ExitCode::FAILURE
        }
    }
}

fn replay_command(mut arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let Some(id) = arguments.next() else {
        eprintln!("replay requires an execution ID");
        print_usage();
        return ExitCode::from(2);
    };
    let Some(id) = id.to_str() else {
        eprintln!("execution ID must be valid UTF-8");
        return ExitCode::from(2);
    };
    let cpu = match arguments.next() {
        None => None,
        Some(flag) if flag == "--cpu" => {
            match arguments.next().as_deref().and_then(|value| value.to_str()) {
                Some(value) if value.chars().all(|character| character.is_ascii_digit()) => {
                    Some(value.to_owned())
                }
                Some(value) => {
                    eprintln!("CPU must be a non-negative integer, found {value:?}");
                    return ExitCode::from(2);
                }
                None => {
                    eprintln!("--cpu requires a non-negative integer");
                    return ExitCode::from(2);
                }
            }
        }
        Some(flag) => {
            eprintln!("unknown replay option {:?}; expected --cpu N", flag);
            return ExitCode::from(2);
        }
    };
    if arguments.next().is_some() {
        eprintln!("replay accepts an execution ID and optional --cpu N");
        return ExitCode::from(2);
    }

    let (path, record) = match find_execution(id) {
        Ok(record) => record,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::FAILURE;
        }
    };
    println!("Replaying {} from {}", record.id, path.display());
    let status = match record.body.recipe_id.as_str() {
        "sort.insertion.uniform.64.correction.v1" => replay_correction(
            &record,
            "sort.insertion.rust.slice.v1",
            "record_sort_correction",
        ),
        "partition.in_place.alternating.64.correction.v1" => replay_correction(
            &record,
            "partition.in_place.rust.slice.v1",
            "record_partition_correction",
        ),
        "sort.uniform.2048.benchmark.v1" => replay_benchmark(&record, cpu.as_deref()),
        recipe => Err(format!(
            "execution recipe {recipe:?} is not replayable by this build"
        )),
    };
    match status {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => {
            eprintln!("replay command exited with {status}");
            ExitCode::FAILURE
        }
        Err(message) => {
            eprintln!("cannot replay {}: {message}", record.id);
            ExitCode::FAILURE
        }
    }
}

fn find_execution(id: &str) -> Result<(PathBuf, ExecutionRecord), String> {
    let directory = Path::new(EXECUTION_DIRECTORY);
    let entries = fs::read_dir(directory).map_err(|error| {
        format!(
            "cannot read {EXECUTION_DIRECTORY}; generate an execution before replaying it: {error}"
        )
    })?;
    for entry in entries {
        let entry =
            entry.map_err(|error| format!("cannot inspect {EXECUTION_DIRECTORY}: {error}"))?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("yaml") {
            continue;
        }
        let contents = fs::read_to_string(&path).map_err(|error| {
            format!(
                "cannot read generated execution {}: {error}",
                path.display()
            )
        })?;
        let record = match ExecutionRecord::from_yaml(&contents) {
            Ok(record) => record,
            Err(error) if contents.lines().any(|line| line == format!("id: {id}")) => {
                return Err(format!(
                    "generated execution {} has an invalid record: {error}",
                    path.display()
                ));
            }
            Err(_) => continue,
        };
        if record.id == id {
            return Ok((path, record));
        }
    }
    Err(format!(
        "execution {id:?} was not found in {EXECUTION_DIRECTORY}; generated observations may have been deleted"
    ))
}

fn replay_correction(
    record: &ExecutionRecord,
    expected_implementation: &str,
    example: &str,
) -> Result<std::process::ExitStatus, String> {
    if record.body.mode != ExecutionMode::Correction
        || record.body.implementation_id != expected_implementation
    {
        return Err("record does not match the versioned correction recipe".to_owned());
    }
    Command::new("cargo")
        .args([
            "run",
            "-q",
            "-p",
            "atlas",
            "--locked",
            "--offline",
            "--example",
            example,
        ])
        .status()
        .map_err(|error| format!("cannot run cargo: {error}"))
}

fn replay_benchmark(
    record: &ExecutionRecord,
    cpu: Option<&str>,
) -> Result<std::process::ExitStatus, String> {
    let cpu = cpu.ok_or_else(|| "benchmark replay requires an explicit --cpu N".to_owned())?;
    if record.body.mode != ExecutionMode::Benchmark
        || !matches!(
            record.body.implementation_id.as_str(),
            "sort.merge.rust.slice.v1"
                | "sort.merge_with_scratch.rust.slice.v1"
                | "sort.insertion.rust.slice.v1"
        )
    {
        return Err("record does not match the versioned sorting benchmark recipe".to_owned());
    }
    Command::new("taskset")
        .args([
            "--cpu-list",
            cpu,
            "cargo",
            "run",
            "--release",
            "-q",
            "-p",
            "atlas-bench",
            "--locked",
            "--offline",
            "--example",
            "record_sort_benchmark",
            "--",
            &record.body.implementation_id,
        ])
        .status()
        .map_err(|error| format!("cannot run taskset: {error}"))
}

fn explain_command(mut arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let Some(id) = arguments.next() else {
        eprintln!("explain requires an implementation ID");
        print_usage();
        return ExitCode::from(2);
    };
    let Some(id) = id.to_str() else {
        eprintln!("implementation ID must be valid UTF-8");
        return ExitCode::from(2);
    };
    if arguments.next().is_some() {
        eprintln!("explain accepts exactly one implementation ID");
        print_usage();
        return ExitCode::from(2);
    }

    match load_registry(Path::new(DEFAULT_REGISTRY)) {
        Ok(registry) => match explain_implementation(&registry, id) {
            Ok(()) => ExitCode::SUCCESS,
            Err(message) => {
                eprintln!("{message}");
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!("{DEFAULT_REGISTRY}: {error}");
            ExitCode::FAILURE
        }
    }
}

fn explain_implementation(registry: &Registry, id: &str) -> Result<(), String> {
    let implementation = registry
        .implementations
        .iter()
        .find(|entity| entity.id == id)
        .ok_or_else(|| format!("implementation {id:?} not found in {DEFAULT_REGISTRY}"))?;
    let algorithm = registry
        .algorithms
        .iter()
        .find(|entity| entity.id == implementation.implements)
        .ok_or_else(|| {
            format!(
                "validated registry invariant failed: algorithm {:?} not found",
                implementation.implements
            )
        })?;
    let problem = registry
        .problems
        .iter()
        .find(|entity| entity.id == algorithm.solves)
        .ok_or_else(|| {
            format!(
                "validated registry invariant failed: problem {:?} not found",
                algorithm.solves
            )
        })?;

    println!("chain:");
    println!("  implementation: {}", implementation.id);
    println!("  algorithm: {}", algorithm.id);
    println!("  problem: {}", problem.id);
    println!("\nimplementation details:");
    print_implementation(implementation);
    println!("\nalgorithm details:");
    print_algorithm(algorithm);
    println!("\nproblem contract:");
    print_problem(problem);
    Ok(())
}

#[derive(Default)]
struct QualificationConstraints {
    stable: bool,
    in_place: bool,
    allocation_none: bool,
}

fn qualify_command(mut arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let Some(problem_id) = arguments.next() else {
        eprintln!("qualify requires a problem ID and at least one constraint");
        print_usage();
        return ExitCode::from(2);
    };
    let Some(problem_id) = problem_id.to_str() else {
        eprintln!("problem ID must be valid UTF-8");
        return ExitCode::from(2);
    };
    let mut constraints = QualificationConstraints::default();
    while let Some(argument) = arguments.next() {
        match argument.to_str() {
            Some("--stable") => constraints.stable = true,
            Some("--in-place") => constraints.in_place = true,
            Some("--allocation") => {
                match arguments.next().as_deref().and_then(|value| value.to_str()) {
                    Some("none") => constraints.allocation_none = true,
                    Some(value) => {
                        eprintln!("unsupported allocation constraint {value:?}; expected none");
                        return ExitCode::from(2);
                    }
                    None => {
                        eprintln!("--allocation requires the value none");
                        return ExitCode::from(2);
                    }
                }
            }
            Some(value) => {
                eprintln!(
                    "unknown qualify constraint {value:?}; expected --stable, --in-place, or --allocation none"
                );
                return ExitCode::from(2);
            }
            None => {
                eprintln!("qualify constraints must be valid UTF-8");
                return ExitCode::from(2);
            }
        }
    }
    if !constraints.stable && !constraints.in_place && !constraints.allocation_none {
        eprintln!("qualify requires at least one constraint");
        return ExitCode::from(2);
    }

    match load_registry(Path::new(DEFAULT_REGISTRY)) {
        Ok(registry) => {
            if !registry
                .problems
                .iter()
                .any(|problem| problem.id == problem_id)
            {
                eprintln!("problem {problem_id:?} not found in {DEFAULT_REGISTRY}");
                return ExitCode::FAILURE;
            }
            print_qualified_implementations(&registry, problem_id, &constraints);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{DEFAULT_REGISTRY}: {error}");
            ExitCode::FAILURE
        }
    }
}

fn print_qualified_implementations(
    registry: &Registry,
    problem_id: &str,
    constraints: &QualificationConstraints,
) {
    for implementation in &registry.implementations {
        let Some(algorithm) = registry
            .algorithms
            .iter()
            .find(|algorithm| algorithm.id == implementation.implements)
        else {
            continue;
        };
        if algorithm.solves != problem_id {
            continue;
        }
        let stable = algorithm.stable.as_ref();
        if constraints.stable && !matches!(stable, Some(claim) if claim.value) {
            continue;
        }
        let in_place = algorithm.in_place.as_ref();
        if constraints.in_place && !matches!(in_place, Some(claim) if claim.value) {
            continue;
        }
        if constraints.allocation_none && implementation.effects.value.allocation != "none" {
            continue;
        }

        println!("implementation\t{}", implementation.id);
        println!("algorithm\t{}", algorithm.id);
        if let Some(stable) = stable {
            println!(
                "stable\t{}\t{}\t{}",
                stable.value, stable.level, stable.source
            );
        }
        if let Some(in_place) = in_place {
            println!(
                "in_place\t{}\t{}\t{}",
                in_place.value, in_place.level, in_place.source
            );
        }
        println!(
            "allocation\t{}\t{}\t{}",
            implementation.effects.value.allocation,
            implementation.effects.level,
            implementation.effects.source
        );
    }
}

fn search_command(mut arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let Some(term) = arguments.next() else {
        eprintln!("search requires a non-empty term");
        print_usage();
        return ExitCode::from(2);
    };
    let Some(term) = term.to_str() else {
        eprintln!("search term must be valid UTF-8");
        return ExitCode::from(2);
    };
    if term.trim().is_empty() {
        eprintln!("search requires a non-empty term");
        return ExitCode::from(2);
    }
    if arguments.next().is_some() {
        eprintln!("search accepts exactly one term");
        print_usage();
        return ExitCode::from(2);
    }

    match load_registry(Path::new(DEFAULT_REGISTRY)) {
        Ok(registry) => {
            print_search_results(&registry, term);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{DEFAULT_REGISTRY}: {error}");
            ExitCode::FAILURE
        }
    }
}

fn print_search_results(registry: &Registry, term: &str) {
    let term = term.to_lowercase();
    for problem in &registry.problems {
        if contains_ignoring_case(&problem.id, &term) {
            print_entity_id("problem", &problem.id);
        }
    }
    for algorithm in &registry.algorithms {
        if contains_ignoring_case(&algorithm.id, &term)
            || contains_ignoring_case(&algorithm.name.value, &term)
        {
            print_entity_id("algorithm", &algorithm.id);
        }
    }
    for implementation in &registry.implementations {
        if contains_ignoring_case(&implementation.id, &term) {
            print_entity_id("implementation", &implementation.id);
        }
    }
}

fn contains_ignoring_case(value: &str, lowercase_term: &str) -> bool {
    value.to_lowercase().contains(lowercase_term)
}

fn print_entity_id(kind: &str, id: &str) {
    println!("{kind}\t{id}");
}

fn show_command(mut arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let Some(id) = arguments.next() else {
        eprintln!("show requires an entity ID");
        print_usage();
        return ExitCode::from(2);
    };
    let Some(id) = id.to_str() else {
        eprintln!("entity ID must be valid UTF-8");
        return ExitCode::from(2);
    };
    if arguments.next().is_some() {
        eprintln!("show accepts exactly one entity ID");
        print_usage();
        return ExitCode::from(2);
    }

    match load_registry(Path::new(DEFAULT_REGISTRY)) {
        Ok(registry) if print_entity(&registry, id) => ExitCode::SUCCESS,
        Ok(_) => {
            eprintln!("entity {id:?} not found in {DEFAULT_REGISTRY}");
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("{DEFAULT_REGISTRY}: {error}");
            ExitCode::FAILURE
        }
    }
}

fn print_entity(registry: &Registry, id: &str) -> bool {
    if let Some(problem) = registry.problems.iter().find(|entity| entity.id == id) {
        print_problem(problem);
    } else if let Some(algorithm) = registry.algorithms.iter().find(|entity| entity.id == id) {
        print_algorithm(algorithm);
    } else if let Some(implementation) = registry
        .implementations
        .iter()
        .find(|entity| entity.id == id)
    {
        print_implementation(implementation);
    } else {
        return false;
    }
    true
}

fn print_problem(problem: &Problem) {
    println!("type: problem");
    println!("id: {}", problem.id);
    print_claim("input", &problem.input);
    if let Some(requires) = &problem.requires {
        print_list_claim("requires", requires);
    }
    print_claim("output", &problem.output);
    print_list_claim("ensures", &problem.ensures);
}

fn print_algorithm(algorithm: &Algorithm) {
    println!("type: algorithm");
    println!("id: {}", algorithm.id);
    println!("solves: {}", algorithm.solves);
    print_claim("name", &algorithm.name);
    if let Some(requires) = &algorithm.requires {
        print_list_claim("requires", requires);
    }
    if let Some(stable) = &algorithm.stable {
        print_claim("stable", stable);
    }
    print_claim("deterministic", &algorithm.deterministic);
    if let Some(in_place) = &algorithm.in_place {
        print_claim("in_place", in_place);
    }
    print_claim("time_worst", &algorithm.time_worst);
    if let Some(time_expected) = &algorithm.time_expected {
        print_claim("time_expected", time_expected);
    }
    print_claim("auxiliary_memory", &algorithm.auxiliary_memory);
}

fn print_implementation(implementation: &Implementation) {
    println!("type: implementation");
    println!("id: {}", implementation.id);
    println!("implements: {}", implementation.implements);
    print_claim("language", &implementation.language);
    print_claim("version", &implementation.version);
    print_claim("license", &implementation.license);
    print_claim("target", &implementation.target);
    print_list_claim("dependencies", &implementation.dependencies);
    print_claim("abi", &implementation.abi);
    print_claim("entrypoint", &implementation.entrypoint);
    print_claim("signature", &implementation.signature);
    print_effects_claim("effects", &implementation.effects);
    print_list_claim("tests", &implementation.tests);
}

fn print_claim<T: Display>(name: &str, claim: &Claim<T>) {
    println!("{name}:");
    println!("  value: {}", claim.value);
    print_evidence(claim);
}

fn print_list_claim(name: &str, claim: &Claim<Vec<String>>) {
    println!("{name}:");
    println!("  value:");
    for item in &claim.value {
        println!("    - {item}");
    }
    print_evidence(claim);
}

fn print_effects_claim(name: &str, claim: &Claim<Effects>) {
    println!("{name}:");
    println!("  value:");
    if claim.value.mutates.is_empty() {
        println!("    mutates: none");
    } else {
        println!("    mutates: {}", claim.value.mutates.join(", "));
    }
    println!("    io: {}", claim.value.io);
    println!("    blocking: {}", claim.value.blocking);
    println!("    allocation: {}", claim.value.allocation);
    print_evidence(claim);
}

fn print_evidence<T>(claim: &Claim<T>) {
    println!("  level: {}", claim.level);
    println!("  source: {}", claim.source);
}

fn validate_command(mut arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let path = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_REGISTRY));
    if arguments.next().is_some() {
        eprintln!("validate accepts at most one path");
        print_usage();
        return ExitCode::from(2);
    }

    validate(&path)
}

fn list_command(mut arguments: impl Iterator<Item = std::ffi::OsString>) -> ExitCode {
    let kind = match arguments.next() {
        None => ListKind::All,
        Some(value) => match value.to_str().and_then(ListKind::parse) {
            Some(kind) => kind,
            None => {
                eprintln!(
                    "unknown entity kind {:?}; expected problem, algorithm, or implementation",
                    value
                );
                return ExitCode::from(2);
            }
        },
    };
    if arguments.next().is_some() {
        eprintln!("list accepts at most one entity kind");
        print_usage();
        return ExitCode::from(2);
    }

    match load_registry(Path::new(DEFAULT_REGISTRY)) {
        Ok(registry) => {
            print_entities(&registry, kind);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{DEFAULT_REGISTRY}: {error}");
            ExitCode::FAILURE
        }
    }
}

#[derive(Clone, Copy)]
enum ListKind {
    All,
    Problem,
    Algorithm,
    Implementation,
}

impl ListKind {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "problem" => Some(Self::Problem),
            "algorithm" => Some(Self::Algorithm),
            "implementation" => Some(Self::Implementation),
            _ => None,
        }
    }
}

fn print_entities(registry: &Registry, kind: ListKind) {
    if matches!(kind, ListKind::All | ListKind::Problem) {
        for problem in &registry.problems {
            print_entity_id("problem", &problem.id);
        }
    }
    if matches!(kind, ListKind::All | ListKind::Algorithm) {
        for algorithm in &registry.algorithms {
            print_entity_id("algorithm", &algorithm.id);
        }
    }
    if matches!(kind, ListKind::All | ListKind::Implementation) {
        for implementation in &registry.implementations {
            print_entity_id("implementation", &implementation.id);
        }
    }
}

fn validate(path: &Path) -> ExitCode {
    match load_registry(path) {
        Ok(registry) => {
            println!(
                "Validated {} problem(s), {} algorithm(s), {} implementation(s), {} execution(s) in {}",
                registry.problems.len(),
                registry.algorithms.len(),
                registry.implementations.len(),
                registry.executions.len(),
                path.display()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{}: {error}", path.display());
            ExitCode::FAILURE
        }
    }
}

fn print_usage() {
    eprintln!(
        "Usage:\n  atlas validate [PATH]\n  atlas list [problem|algorithm|implementation]\n  atlas show <id>\n  atlas search <term>\n  atlas explain <implementation-id>\n  atlas qualify <problem-id> [--stable] [--in-place] [--allocation none]\n  atlas replay <execution-id> [--cpu N]\n  atlas compare <execution-id> <execution-id>...\n  atlas compose cleanup [--goal expected-time] [--force ID|--forbid ID] [--rust]\n  atlas compose find [--force ID|--forbid ID] [--rust]\n  atlas index [DB_PATH]"
    );
}
