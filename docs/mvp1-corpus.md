# MVP 1 target corpus

Accepted by DEC-011 and DEC-012.

Status: **complete**.

| Problem | Algorithms | Implementations |
|---|---|---:|
| `sequence.sort` | top-down merge; stable insertion | 3 |
| `sequence.search` | linear; binary lower-bound | 2 |
| `sequence.minimum` | linear scan | 1 |
| `sequence.maximum` | linear scan | 1 |
| `sequence.filter` | stable copy; stable in-place compaction | 3 |
| `sequence.partition` | stable copy; unstable in-place partition | 3 |
| `sequence.reverse` | symmetric in-place swaps | 1 |
| `sequence.merge_sorted` | linear two-input merge | 2 |
| `sequence.is_sorted` | adjacent comparison | 1 |
| `sequence.deduplicate` | stable quadratic; stable hash-based | 3 |
| **Total** | **15** | **20** |

## Operational variants

The five implementations beyond one implementation per algorithm are:

- merge sort with internal or caller-provided scratch storage;
- copying filter with allocated or caller-provided output;
- stable partition with allocated or caller-provided outputs;
- sorted merge with allocated or caller-provided output;
- hash-based deduplication with allocated or caller-provided output.

Each variant must have an observable signature, mutation, or allocation-policy
difference. Scalar wrappers do not count toward the target.
