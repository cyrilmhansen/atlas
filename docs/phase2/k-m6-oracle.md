# K-M6 exact bounded top-k oracle

Status: frozen orchestrator-only oracle; never included in selection packets

Date: 2026-07-15

## Correct selection

- Problem: `stream.top_k`;
- Algorithm: `stream.top_k.min_heap`;
- Implementation: `stream.top_k.rust.std_binary_heap.v1`.

Required evidence distinctions:

- input/output are `declared` in the problem contract;
- exact occurrence, omission and zero-capacity guarantees are `tested`;
- determinism is algorithm-level `declared`;
- worst time `O(n log k)` and retained memory `O(k)` are `inferred`;
- the implementation target is `tested` and requires Rust `std`;
- retained heap and output allocations are `tested` effects;
- the Rust ABI and entry point are only `declared`.

The response must not promote these claims to `proven`, promise sorted semantic
output, claim allocation freedom or claim `no_std` compatibility.

## Alternative adjudication

At least three of these distinctions should appear:

- reservoir sampling: reject because it selects a random subset rather than
  greatest ordered occurrences;
- Misra-Gries/heavy hitters: reject because frequency candidates or verified
  heavy hitters are not greatest values under a total order;
- Bloom membership: reject because it answers approximate membership and has a
  one-sided error contract;
- whole-input sorting: reject under the retained-value bound because it
  materializes or mutates the full input rather than retaining `O(k)` stream
  values;
- online moments: reject because its output is aggregate numeric moments.

An alternative absent from the arm's packet may be marked unresolved rather
than invented.

## Interface oracle

The assisted arm can discover the exact chain with:

```text
atlas search top-k
atlas show stream.top_k
atlas show stream.top_k.min_heap
atlas explain stream.top_k.rust.std_binary_heap.v1
```

`qualify` cannot express exactness, multiplicity, retained `O(k)` memory or
streaming and requires one of its fixed constraints. No `compose` scenario
covers this task. A response claiming generic qualification or composition is
incorrect.

## Post-reveal oracle

Reveal only after both initial responses and end times are frozen:

- `crates/atlas/tests/external_streaming_adapters.rs` at the packet digest;
- the command and result of
  `cargo test -p atlas --test external_streaming_adapters bounded_top_k_is_exact_and_never_exceeds_its_budget --locked`.

The test covers duplicate maxima, `k > n`, `k = 0`, exact fixture output and the
retained-size behavior stated by its name. It does not prove all inputs,
asymptotic cost or the standard library implementation internals.

## Comparison rubric

Compare arms without collapsing uncertainty into failure:

| Dimension | Observation |
|---|---|
| identity | exact three IDs, partial identity, or unresolved |
| contract correctness | satisfied, contradicted or invented requirements |
| evidence discipline | levels and source boundaries preserved or promoted |
| alternatives | decision-relevant rejections versus generic guesses |
| interface insight | query limitations correctly identified |
| correction | initial selection retained/corrected after reveal |
| intervention | number, content and elapsed handling time |

The experiment asks whether Atlas changes correctness, explanation quality or
human intervention. Merely producing a plan is not success.
