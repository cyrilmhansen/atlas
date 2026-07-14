# K-M0 current model baseline

Snapshot date: 2026-07-14  
Repository baseline: Phase 2 activation commit `3a2aa44`  
Purpose: record capabilities before any external-corpus import

This snapshot is descriptive. It prevents later improvements from being
mistaken for capabilities that existed when import friction was observed.

## Authoritative registry

Schema 0.1 is one in-memory UTF-8 YAML document with separate `Problem`,
`Algorithm`, `Implementation` and empty `Execution` collections. Unknown fields
are rejected and identifiers are globally unique.

### Expressible directly

- problem input, optional requirements, output and postconditions;
- one `Algorithm.solves` problem reference;
- algorithm name, extra requirements, determinism, worst/expected time and
  auxiliary-memory claims;
- optional stability and in-place claims;
- one `Implementation.implements` algorithm reference;
- implementation language, version, license, target, dependencies, ABI,
  entrypoint, signature, effects and tests;
- evidence level and one textual source expression for every property claim;
- implementation mutation, I/O, blocking and allocation effects.

### Not structurally expressible

- bibliographic works, authors, editions, pages, commits and licenses as source
  entities;
- aliases, terminology mappings or source-local identities;
- algorithm variant, derivation, equivalence or domination relations;
- one algorithm solving several problem contracts;
- typed graph, stateful-data-structure or streaming contracts;
- amortized cost per operation, parameterized operation families or mixed
  complexity regimes;
- approximation, probability, numerical error and machine arithmetic domains;
- persistent state transitions, representation invariants and query mutation;
- structured transformation provenance between source and Atlas artifacts.

Free-text claims may document many of these facts but cannot make them generic
selection predicates. Their presence in prose is not structural support.

## Evidence and source resolution

Every non-structural field is a claim at `declared`, `inferred`, `tested`,
`observed` or `proven` level. The current source grammar resolves selected local
files, implementations and Rust tests. Documentary schemes are syntax-checked;
the registry does not fetch or validate external bibliographic references.

The K-M0 worksheet therefore carries source identity and fidelity outside the
public schema. This is a deliberate experimental record, not a competing
authority.

## Query and composition behavior

The CLI can validate, list, show, search, explain and qualify registry entries.
Qualification filters the properties already modeled by schema 0.1.

Composition is not generic candidate discovery. `crates/atlas/src/composition.rs`
contains five reviewed, scenario-specific candidate pairs with hard-coded
implementation identifiers. `--force` and `--forbid` operate only within those
known candidates. A new manifest cannot currently enter a composition candidate
set without code changes.

This limitation is expected Phase 2 evidence and must not be hidden during the
pilot. Generic discovery is tested later at K-M5.

## Experimental algorithm AST

The private AST currently provides:

- scalar types for booleans, elements, optional elements, indices, orderings,
  comparators and predicates;
- sequence and range types;
- structured `let`, operation, `if`, `while`, `for each`, `return` and `break`;
- semantic operations for read, write, compare, swap, recurse, allocate, copy,
  partition, predicate and assert;
- parameter access modes and summaries of mutation, allocation and copy;
- typed sequence indexing, ranges, length and a small arithmetic/comparison
  expression set;
- reviewed builders for sequence-oriented algorithms and two private textual
  pseudocode experiments.

It does not directly model graph/node/edge values, adjacency, priority queues,
sets, mutable object state, numeric weights, maps, parent forests or amortized
operation sequences. Generic calls can name an operation but do not provide a
typed semantic model for these concepts.

AST coverage is explicitly not required for an external import. Importers record
whether it would be exact, lossy or absent; Execution Lab remains frozen.

## Corpus baseline

- 10 problems;
- 15 algorithms;
- 20 Rust implementations;
- all concentrated in sequence operations;
- five bounded sequence composition scenarios;
- no external graph, dynamic-structure or streaming entity yet.

## Baseline predictions to test

| Pilot subject | Expected exact support | Expected friction |
|---|---|---|
| BFS | free-text problem/algorithm contract and complexity | traversal versus path-materialization identity; graph types; persistent frontier |
| Dijkstra | nonnegative-weight requirement and high-level complexity text | numerical domain, output variant, early goal, priority-queue effects |
| Union-find | high-level operation names and memory text | stateful multi-operation contract, mutating lookup, amortized cost and representation invariant |

These are falsifiable predictions, not accepted model changes. The independent
worksheets and equivalence matrix determine what actually fails.

## Change control during the pilot

- no public schema change;
- no AST extension;
- no new importer or agent API;
- no registry entity before independent worksheets are compared;
- no claim that free-text documentation supports generic selection;
- any baseline correction records whether it was a documentation error or an
  actual post-baseline capability change.
