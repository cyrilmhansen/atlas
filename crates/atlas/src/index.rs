use std::fmt;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use rusqlite::{Connection, Transaction, params};
use sha2::{Digest, Sha256};

use crate::registry::{Claim, CostProfile, Effects, Registry};

pub const PROJECTION_VERSION: &str = "2";

#[derive(Debug, Eq, PartialEq)]
pub struct ProjectionSummary {
    pub entities: usize,
    pub relations: usize,
    pub claims: usize,
    pub digest: String,
}

#[derive(Debug)]
pub enum IndexError {
    CreateDirectory(std::io::Error),
    Sql(rusqlite::Error),
}

impl fmt::Display for IndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateDirectory(error) => {
                write!(formatter, "cannot create database directory: {error}")
            }
            Self::Sql(error) => write!(formatter, "SQLite projection failed: {error}"),
        }
    }
}

impl From<rusqlite::Error> for IndexError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sql(error)
    }
}

pub fn rebuild_database(
    registry: &Registry,
    database_path: &Path,
) -> Result<ProjectionSummary, IndexError> {
    if let Some(parent) = database_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(IndexError::CreateDirectory)?;
    }
    let mut connection = Connection::open(database_path)?;
    rebuild_connection(registry, &mut connection)
}

pub fn summarize_registry(registry: &Registry) -> Result<ProjectionSummary, IndexError> {
    let mut connection = Connection::open_in_memory()?;
    rebuild_connection(registry, &mut connection)
}

fn rebuild_connection(
    registry: &Registry,
    connection: &mut Connection,
) -> Result<ProjectionSummary, IndexError> {
    connection.execute_batch("PRAGMA foreign_keys = ON;")?;
    let transaction = connection.transaction()?;
    create_schema(&transaction)?;
    let mut counts = Counts::default();

    transaction.execute(
        "INSERT INTO projection_meta(key, value) VALUES ('projection_version', ?1)",
        [PROJECTION_VERSION],
    )?;
    transaction.execute(
        "INSERT INTO projection_meta(key, value) VALUES ('source_schema_version', ?1)",
        [&registry.schema_version],
    )?;

    let mut entity_ordinal = 0_i64;
    for condition in &registry.conditions {
        insert_entity(&transaction, &condition.id, "condition", entity_ordinal)?;
        entity_ordinal += 1;
        counts.entities += 1;
        insert_claim(
            &transaction,
            &condition.id,
            "statement",
            encode_string(&condition.statement.value),
            &condition.statement,
            0,
        )?;
        counts.claims += 1;
    }
    for problem in &registry.problems {
        insert_entity(&transaction, &problem.id, "problem", entity_ordinal)?;
        entity_ordinal += 1;
        counts.entities += 1;
        let mut ordinal = 0_i64;
        insert_claim(
            &transaction,
            &problem.id,
            "input",
            encode_string(&problem.input.value),
            &problem.input,
            ordinal,
        )?;
        ordinal += 1;
        if let Some(requires) = &problem.requires {
            insert_claim(
                &transaction,
                &problem.id,
                "requires",
                encode_list(&requires.value),
                requires,
                ordinal,
            )?;
            ordinal += 1;
        }
        insert_claim(
            &transaction,
            &problem.id,
            "output",
            encode_string(&problem.output.value),
            &problem.output,
            ordinal,
        )?;
        ordinal += 1;
        insert_claim(
            &transaction,
            &problem.id,
            "ensures",
            encode_list(&problem.ensures.value),
            &problem.ensures,
            ordinal,
        )?;
        counts.claims += ordinal as usize + 1;
    }

    for algorithm in &registry.algorithms {
        insert_entity(&transaction, &algorithm.id, "algorithm", entity_ordinal)?;
        entity_ordinal += 1;
        counts.entities += 1;
        insert_relation(&transaction, &algorithm.id, "solves", &algorithm.solves, 0)?;
        counts.relations += 1;

        let mut ordinal = 0_i64;
        insert_claim(
            &transaction,
            &algorithm.id,
            "name",
            encode_string(&algorithm.name.value),
            &algorithm.name,
            ordinal,
        )?;
        ordinal += 1;
        if let Some(requires) = &algorithm.requires {
            insert_claim(
                &transaction,
                &algorithm.id,
                "requires",
                encode_list(&requires.value),
                requires,
                ordinal,
            )?;
            ordinal += 1;
        }
        if let Some(stable) = &algorithm.stable {
            insert_claim(
                &transaction,
                &algorithm.id,
                "stable",
                encode_bool(stable.value),
                stable,
                ordinal,
            )?;
            ordinal += 1;
        }
        insert_claim(
            &transaction,
            &algorithm.id,
            "deterministic",
            encode_bool(algorithm.deterministic.value),
            &algorithm.deterministic,
            ordinal,
        )?;
        ordinal += 1;
        if let Some(in_place) = &algorithm.in_place {
            insert_claim(
                &transaction,
                &algorithm.id,
                "in_place",
                encode_bool(in_place.value),
                in_place,
                ordinal,
            )?;
            ordinal += 1;
        }
        for (cost_index, cost) in algorithm.costs.iter().enumerate() {
            let path = format!("costs[{cost_index}]");
            insert_claim(
                &transaction,
                &algorithm.id,
                &path,
                encode_cost(&cost.value),
                cost,
                ordinal,
            )?;
            ordinal += 1;
            for (requirement_index, condition) in cost.value.requires.iter().enumerate() {
                insert_relation(
                    &transaction,
                    &algorithm.id,
                    &format!("{path}.requires"),
                    condition,
                    requirement_index as i64,
                )?;
                counts.relations += 1;
            }
        }
        counts.claims += ordinal as usize;
    }

    for implementation in &registry.implementations {
        insert_entity(
            &transaction,
            &implementation.id,
            "implementation",
            entity_ordinal,
        )?;
        entity_ordinal += 1;
        counts.entities += 1;
        insert_relation(
            &transaction,
            &implementation.id,
            "implements",
            &implementation.implements,
            0,
        )?;
        counts.relations += 1;

        let claims = [
            (
                "language",
                encode_string(&implementation.language.value),
                &implementation.language,
            ),
            (
                "version",
                encode_string(&implementation.version.value),
                &implementation.version,
            ),
            (
                "license",
                encode_string(&implementation.license.value),
                &implementation.license,
            ),
            (
                "target",
                encode_string(&implementation.target.value),
                &implementation.target,
            ),
        ];
        let mut ordinal = 0_i64;
        for (path, value, claim) in claims {
            insert_claim(
                &transaction,
                &implementation.id,
                path,
                value,
                claim,
                ordinal,
            )?;
            ordinal += 1;
        }
        insert_claim(
            &transaction,
            &implementation.id,
            "dependencies",
            encode_list(&implementation.dependencies.value),
            &implementation.dependencies,
            ordinal,
        )?;
        ordinal += 1;
        let claims = [
            (
                "abi",
                encode_string(&implementation.abi.value),
                &implementation.abi,
            ),
            (
                "entrypoint",
                encode_string(&implementation.entrypoint.value),
                &implementation.entrypoint,
            ),
            (
                "signature",
                encode_string(&implementation.signature.value),
                &implementation.signature,
            ),
        ];
        for (path, value, claim) in claims {
            insert_claim(
                &transaction,
                &implementation.id,
                path,
                value,
                claim,
                ordinal,
            )?;
            ordinal += 1;
        }
        insert_claim(
            &transaction,
            &implementation.id,
            "effects",
            encode_effects(&implementation.effects.value),
            &implementation.effects,
            ordinal,
        )?;
        ordinal += 1;
        insert_claim(
            &transaction,
            &implementation.id,
            "tests",
            encode_list(&implementation.tests.value),
            &implementation.tests,
            ordinal,
        )?;
        counts.claims += ordinal as usize + 1;
    }

    let digest = logical_digest(&transaction)?;
    transaction.execute(
        "INSERT INTO projection_meta(key, value) VALUES ('logical_digest_sha256', ?1)",
        [&digest],
    )?;
    transaction.commit()?;

    Ok(ProjectionSummary {
        entities: counts.entities,
        relations: counts.relations,
        claims: counts.claims,
        digest,
    })
}

#[derive(Default)]
struct Counts {
    entities: usize,
    relations: usize,
    claims: usize,
}

fn create_schema(transaction: &Transaction<'_>) -> rusqlite::Result<()> {
    transaction.execute_batch(
        "DROP TABLE IF EXISTS claims;
         DROP TABLE IF EXISTS relations;
         DROP TABLE IF EXISTS entities;
         DROP TABLE IF EXISTS projection_meta;

         CREATE TABLE projection_meta (
             key TEXT PRIMARY KEY,
             value TEXT NOT NULL
         ) STRICT;
         CREATE TABLE entities (
             id TEXT PRIMARY KEY,
             kind TEXT NOT NULL CHECK(kind IN ('condition', 'problem', 'algorithm', 'implementation')),
             ordinal INTEGER NOT NULL CHECK(ordinal >= 0)
         ) STRICT;
         CREATE TABLE relations (
             source_id TEXT NOT NULL REFERENCES entities(id),
             relation TEXT NOT NULL,
             target_id TEXT NOT NULL REFERENCES entities(id),
             ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
             PRIMARY KEY(source_id, relation, ordinal)
         ) STRICT;
         CREATE TABLE claims (
             entity_id TEXT NOT NULL REFERENCES entities(id),
             path TEXT NOT NULL,
             value TEXT NOT NULL,
             level TEXT NOT NULL,
             source TEXT NOT NULL,
             ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
             PRIMARY KEY(entity_id, path)
         ) STRICT;",
    )
}

fn insert_entity(
    transaction: &Transaction<'_>,
    id: &str,
    kind: &str,
    ordinal: i64,
) -> rusqlite::Result<()> {
    transaction.execute(
        "INSERT INTO entities(id, kind, ordinal) VALUES (?1, ?2, ?3)",
        params![id, kind, ordinal],
    )?;
    Ok(())
}

fn insert_relation(
    transaction: &Transaction<'_>,
    source_id: &str,
    relation: &str,
    target_id: &str,
    ordinal: i64,
) -> rusqlite::Result<()> {
    transaction.execute(
        "INSERT INTO relations(source_id, relation, target_id, ordinal)
         VALUES (?1, ?2, ?3, ?4)",
        params![source_id, relation, target_id, ordinal],
    )?;
    Ok(())
}

fn insert_claim<T>(
    transaction: &Transaction<'_>,
    entity_id: &str,
    path: &str,
    value: String,
    claim: &Claim<T>,
    ordinal: i64,
) -> rusqlite::Result<()> {
    transaction.execute(
        "INSERT INTO claims(entity_id, path, value, level, source, ordinal)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            entity_id,
            path,
            value,
            claim.level.to_string(),
            claim.source,
            ordinal
        ],
    )?;
    Ok(())
}

fn encode_string(value: &str) -> String {
    format!("s:{}:{value}", value.len())
}

fn encode_bool(value: bool) -> String {
    if value { "b:1" } else { "b:0" }.to_owned()
}

fn encode_list(values: &[String]) -> String {
    let mut encoded = format!("l:{}", values.len());
    for value in values {
        let _ = write!(encoded, ":{}:{value}", value.len());
    }
    encoded
}

fn encode_effects(effects: &Effects) -> String {
    let fields = [
        encode_list(&effects.mutates),
        encode_string(&effects.io),
        encode_bool(effects.blocking),
        encode_string(&effects.allocation),
    ];
    let mut encoded = String::from("e:4");
    for field in fields {
        let _ = write!(encoded, ":{}:{field}", field.len());
    }
    encoded
}

fn encode_cost(cost: &CostProfile) -> String {
    let fields = [
        encode_string(&cost.metric.to_string()),
        encode_string(&cost.regime.to_string()),
        encode_string(&cost.bound),
        encode_list(&cost.requires),
    ];
    let mut encoded = String::from("c:4");
    for field in fields {
        let _ = write!(encoded, ":{}:{field}", field.len());
    }
    encoded
}

fn logical_digest(transaction: &Transaction<'_>) -> rusqlite::Result<String> {
    let mut hasher = Sha256::new();
    hash_query(
        transaction,
        &mut hasher,
        "meta",
        "SELECT key, value FROM projection_meta
         WHERE key <> 'logical_digest_sha256' ORDER BY key",
        2,
    )?;
    hash_query(
        transaction,
        &mut hasher,
        "entities",
        "SELECT id, kind, CAST(ordinal AS TEXT) FROM entities ORDER BY id",
        3,
    )?;
    hash_query(
        transaction,
        &mut hasher,
        "relations",
        "SELECT source_id, relation, target_id, CAST(ordinal AS TEXT)
         FROM relations ORDER BY source_id, relation, ordinal",
        4,
    )?;
    hash_query(
        transaction,
        &mut hasher,
        "claims",
        "SELECT entity_id, path, value, level, source, CAST(ordinal AS TEXT)
         FROM claims ORDER BY entity_id, path, ordinal",
        6,
    )?;

    let digest = hasher.finalize();
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(hex, "{byte:02x}");
    }
    Ok(hex)
}

fn hash_query(
    transaction: &Transaction<'_>,
    hasher: &mut Sha256,
    table: &str,
    query: &str,
    columns: usize,
) -> rusqlite::Result<()> {
    hash_field(hasher, table);
    let mut statement = transaction.prepare(query)?;
    let mut rows = statement.query([])?;
    while let Some(row) = rows.next()? {
        for column in 0..columns {
            let value: String = row.get(column)?;
            hash_field(hasher, &value);
        }
    }
    Ok(())
}

fn hash_field(hasher: &mut Sha256, value: &str) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::{PROJECTION_VERSION, rebuild_connection};
    use crate::registry::Registry;
    use rusqlite::Connection;

    const REGISTRY: &str = include_str!("../../../registry/atlas.yaml");

    fn registry() -> Registry {
        serde_yaml::from_str(REGISTRY).expect("committed registry must parse")
    }

    #[test]
    fn projects_expected_rows_and_metadata() {
        let mut connection = Connection::open_in_memory().unwrap();

        let summary = rebuild_connection(&registry(), &mut connection).unwrap();

        assert_eq!(summary.entities, 115);
        assert!(summary.relations > 82);
        assert!(summary.claims > 650);
        assert_eq!(summary.digest.len(), 64);
        let projection_version: String = connection
            .query_row(
                "SELECT value FROM projection_meta WHERE key = 'projection_version'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(projection_version, PROJECTION_VERSION);
        let versions: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM claims WHERE path = 'version'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(versions, 43);
        let cost_profiles: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM claims WHERE path LIKE 'costs[%]%'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(cost_profiles > 78);
    }

    #[test]
    fn repeated_rebuilds_have_identical_rows_and_digest() {
        let mut connection = Connection::open_in_memory().unwrap();
        let first = rebuild_connection(&registry(), &mut connection).unwrap();
        let second = rebuild_connection(&registry(), &mut connection).unwrap();

        assert_eq!(first, second);
        let stored: String = connection
            .query_row(
                "SELECT value FROM projection_meta WHERE key = 'logical_digest_sha256'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(stored, second.digest);
    }

    #[test]
    fn logical_change_changes_digest() {
        let mut first_registry = registry();
        let mut second_registry = registry();
        second_registry.problems[0].input.source = "changed:source".to_owned();
        let mut first_connection = Connection::open_in_memory().unwrap();
        let mut second_connection = Connection::open_in_memory().unwrap();

        let first = rebuild_connection(&first_registry, &mut first_connection).unwrap();
        let second = rebuild_connection(&second_registry, &mut second_connection).unwrap();

        assert_ne!(first.digest, second.digest);
        first_registry.problems[0].input.source = "changed:source".to_owned();
        let rebuilt = rebuild_connection(&first_registry, &mut first_connection).unwrap();
        assert_eq!(rebuilt.digest, second.digest);
    }
}
