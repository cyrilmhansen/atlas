# K4-M3 exact bounded top-k source review

Status: complete; `topk-relaxed-A` implemented and adjudicated

Date: 2026-07-17

## Target

Add a foreign exact implementation for the existing `stream.top_k` Problem.
The candidate must retain occurrences rather than distinct values, handle zero
capacity, preserve exactly the greatest `min(k, n)` occurrences and expose its
output order. It should change the current minimum-heap tradeoff rather than
merely wrap another binary heap.

The frozen requests remain `top_k.exact_bounded`, `top_k.no_allocation`,
`top_k.n_log_k_worst` and `top_k.sorted_output`.

## Option A - itertools 0.15.0 k_largest_relaxed

Use
[`Itertools::k_largest_relaxed`](https://docs.rs/itertools/0.15.0/itertools/trait.Itertools.html#method.k_largest_relaxed).
It consumes an iterator and returns the exact `k` largest elements in descending
order. Its implementation delegates to the relaxed selection path: retain up
to `2k` elements, apply `select_nth_unstable` when the buffer fills, truncate
to `k`, and sort the final result.

The upstream contract for the corresponding relaxed selection states
`2 * k * sizeof(Item) + O(1)` memory and `O(n + k log k)` time. This changes
the current tradeoff deliberately: twice the retained element capacity buys
linear processing in `n`, while preserving a sorted result.

Exact package metadata verified with `cargo info itertools@0.15.0`:

- MIT OR Apache-2.0;
- Rust 1.63 minimum, compatible with the workspace Rust 1.85 baseline;
- `no_std` plus allocation through `default-features = false` and
  `features = ["use_alloc"]`;
- one mandatory, default-feature-free `either` dependency.

Cost:

- one pinned dev-dependency plus its small transitive `either` dependency;
- one Algorithm, one Implementation and one focused direct-upstream test;
- this living experiment record and one unchanged-evaluator overlay input.

Risks:

- `O(n + k log k)` is decision-theoretically no worse than `O(n log k)` in the
  nontrivial `k >= 2` regime, but the K-M5 evaluator compares opaque cost
  strings and cannot prove that relation;
- the implementation may allocate capacity for `2k`, so `O(k)` asymptotic
  retained memory holds while the constant-factor tradeoff remains material;
- `k_largest_relaxed` was added to a broad utility crate rather than a
  top-k-specific package, so only its narrow entry point is admitted.

Reversibility: high. The dependency is test-only and the two entities remain
isolated from production Atlas APIs.

## Option B - itertools 0.15.0 k_largest

Use the adjacent exact `k_largest` method. It guarantees descending output and
bounded storage, but implements a custom binary heap and is semantically the
same strategy as the existing candidate.

Cost and risks:

- the dependency and adapter cost are essentially identical to Option A;
- source and API independence improve, but structural diversity does not;
- the batch would mostly retest a known `O(n log k)` / `O(k)` design and would
  provide little evidence about selection vocabulary transfer.

Reversibility: high; information gain low.

## Option C - standard-library select_nth_unstable on a full buffer

Collect the finite stream into a `Vec`, partition it with
`slice::select_nth_unstable`, retain the greatest `k`, and sort those results.
This uses maintained foreign standard-library selection code and adds no
dependency.

Cost and risks:

- requires `O(n)` buffered input rather than `O(k)` retained state;
- Atlas must author the collection, boundary and output adapter;
- it is an exact batch selection candidate, but fails the frozen
  `top_k.exact_bounded` memory requirement and weakens the streaming test.

Reversibility: high. It is useful as a future negative control, not as the sole
K4-M3 competitor.

## Recommendation

Recommend **Option A (`topk-relaxed-A`)**.

It is the first candidate that is simultaneously foreign, exact, bounded by
`O(k)`, explicitly sorted and structurally distinct from the current heap. It
also creates a useful falsifier: the documentary decision can recognize
`O(n + k log k)` as satisfying the frozen upper bound, while the unchanged
evaluator should expose its opaque-bound comparison limit rather than accept it
through special-case code.

## Minimum experiment

1. Pin itertools 0.15.0 with only `use_alloc`.
2. Execute `k_largest_relaxed` directly on empty, singleton, duplicate-heavy,
   `k = 0`, `k = 1`, `k > n`, ascending and descending inputs.
3. Assert exact multiplicity and descending output against a full-sort oracle.
4. Discover both candidates through `solves` and `implements`, never IDs.
5. Adjudicate the four frozen requests with documentary outcome classes and
   the unchanged K-M5 evaluator.
6. Stop after the two-candidate matrix; add no cost algebra or schema field.

Owner choice: `topk-relaxed-A`.

## Result

The pinned, allocation-only itertools implementation is registered as
`stream.top_k.relaxed_selection` and
`stream.top_k.itertools.relaxed.0_15_0`. Atlas calls
`k_largest_relaxed` directly. Tests compare it with a full descending sort for
empty, singleton, duplicate-heavy, ascending, descending and mixed inputs at
`k = 0`, `k = 1`, ordinary `k` and `k > n`.

Registry relations discover exactly two Algorithms and two Implementations for
`stream.top_k`; the discovery test contains no candidate identifier branch.

## Frozen requests

| Request | Minimum heap | Relaxed selection | Verdict |
|---|---|---|---|
| `top_k.exact_bounded` | accepted | accepted | both preserve exact multiplicity with `O(k)` retained elements |
| `top_k.no_allocation` | rejected | rejected | both allocate bounded retained/output storage |
| `top_k.n_log_k_worst` | accepted | unsupported by evaluator | documentary analysis accepts `O(n + k log k)` as no worse than `O(n log k)` for nontrivial `2 <= k <= n`; the evaluator only matches identical cost strings |
| `top_k.sorted_output` | accepted | accepted | both tests and the itertools contract establish descending output |

The unchanged evaluator produces the expected missing-exact-cost reason for
the relaxed candidate. No algebraic rule, special case or evaluator extension
was added. Boundary cases `k < 2` are established behaviorally rather than by
forcing a degenerate two-parameter asymptotic expression.

## Verdict

K4-M3 passes. Atlas can ingest and discover a foreign strategy with a genuinely
different time/memory tradeoff, then reuse the same exactness, effect, memory
and output-order vocabulary. Its remaining failure is query normalization of
equivalent or dominating cost expressions, not missing top-k knowledge. That
single limitation does not justify extending the evaluator or public schema.
