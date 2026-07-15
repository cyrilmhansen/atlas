# K-M6 selection task - exact bounded top-k

Status: frozen common task

You are selecting an existing Atlas component before seeing implementation
source code. Do not implement an algorithm during this stage.

## Required behavior

For a finite stream of signed 32-bit integers and a nonnegative capacity `k`,
select a component that returns the greatest `min(k, n)` input occurrences.

- Duplicate occurrences are significant and must be preserved.
- No omitted occurrence may be greater than a retained occurrence.
- `k = 0` must return an empty result.
- The semantic result need not promise an output order.
- Selection must be deterministic for a fixed input and `k`.
- The algorithm may retain at most `O(k)` stream values, excluding its returned
  result.
- A Rust implementation using `std` and allocating `O(k)` retained/output
  storage is acceptable.
- Approximate, probabilistic, frequency-based and whole-input-sort solutions are
  incompatible with this request.

## Selection-stage deliverable

Complete `RESPONSE.md` using the supplied template. State:

1. the selected Problem, Algorithm and Implementation identities, or explicitly
   state which identity cannot be established from your packet;
2. the contract and complexity evidence supporting the selection;
3. at least three plausible alternatives and why each is accepted, rejected or
   unresolved;
4. every relevant uncertainty or information gap;
5. whether the available interface could perform a generic qualification or
   composition for this task.

Do not claim to have executed implementation tests or inspected source during
selection. Do not infer `no_std`, allocation-free, sorted output, proof-level
correctness or a stable ABI unless your packet provides explicit evidence.
