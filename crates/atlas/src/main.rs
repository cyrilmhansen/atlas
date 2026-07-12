use std::env;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use atlas::index::rebuild_database;
use atlas::registry::{
    Algorithm, Claim, Effects, Implementation, Problem, Registry, load_registry,
};

const DEFAULT_REGISTRY: &str = "registry/atlas.yaml";
const DEFAULT_DATABASE: &str = "build/atlas.sqlite3";

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
        Some("index") => index_command(arguments),
        _ => {
            eprintln!("unknown command {:?}", command);
            print_usage();
            ExitCode::from(2)
        }
    }
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
        "Usage:\n  atlas validate [PATH]\n  atlas list [problem|algorithm|implementation]\n  atlas show <id>\n  atlas search <term>\n  atlas explain <implementation-id>\n  atlas index [DB_PATH]"
    );
}
