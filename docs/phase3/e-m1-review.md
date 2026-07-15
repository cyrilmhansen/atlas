# E-M1 relational catalog review

Status: complete on 2026-07-15

Authority: DEC-071 and `docs/phase3-explorer.md`

## Question

Can a visitor inspect the complete registry as connected, sourced knowledge and
compare two entities factually without reading YAML or turning the private Web
projection into a new public schema?

## Delivered slice

The static Explorer now provides:

- text and kind filtering over all 31 problems, 36 algorithms and 40
  implementations;
- selectable entity detail with every existing schema 0.1 claim projected with
  its evidence level and provenance;
- exact Problem -> Algorithm -> Implementation navigation and reverse paths;
- same-kind side-by-side comparison, including an explicit `Not recorded`
  state for absent claims;
- a handoff to the existing execution workbench for exactly the five algorithms
  with accepted generated presentations;
- a bounded, scrollable result list and full-width responsive detail and
  comparison layouts.

`web/catalog.mjs` contains the private pure catalog operations. It does not
define registry identity, infer relations, normalize claims or select a
candidate. Those facts continue to come from the validated YAML registry and
the Rust projection.

## Acceptance evidence

| Criterion | Result |
|---|---|
| Every projected entity is searchable | Passed for all 107 identities |
| Exact relation navigation | Passed in forward and reverse directions |
| Sourced claims only | Passed; values, levels and provenance are projected together |
| Missing comparison facts | Passed; rendered as absent, never inferred |
| Ranking boundary | Passed; no score, winner or dominance conclusion exists |
| Execution availability | Passed; five available presentations and explicit absence elsewhere |
| Existing execution behavior | Passed by the retained native/WASM and differential gates |
| Responsive presentation | Inspected at 390, 768, 1440 and 1920 pixels |
| Static reproducibility | Passed through `scripts/check-web.sh` |

Verification commands:

```sh
scripts/check-web.sh
cargo test -p atlas --locked
git diff --check
```

Useful local routes after `scripts/build-web.sh`:

```text
/?entity=graph.bfs.traversal
/?entity=deduplicate.hash.stable&compare=deduplicate.quadratic.stable
```

## Authority boundary

The additional implementation metadata and effect claims in
`atlas-web-private-v0` are a derived display projection of fields already
present in schema 0.1. No public field, persistent format, protocol, ABI,
runtime instruction or registry relation changed.

The comparison is intentionally syntactic and evidential. Equal-looking values
do not establish substitutability, and absent claims do not establish a negative
property. The execution button establishes only that a reviewed bounded visual
program exists; it does not promote local counts or timings into theoretical
evidence.

## Limits and next falsifier

E-M1 proves reachability and faithful display, not visitor comprehension. It
also does not test publication, qualified selection or new executable coverage.

E-M2 should therefore begin with a small usability protocol: ask a visitor to
inspect one executable sequence algorithm and one non-executable foreign-corpus
algorithm, identify which statements are sourced theory, tested behavior and
local observation, and follow the exact implementation chain. Only observed
handoff failures should justify further UI changes. Execution Lab remains
frozen.
