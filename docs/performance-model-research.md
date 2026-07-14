# Performance modeling beyond wall-clock time

Status: research report, 2026-07-14. This document is non-normative and does
not change the Atlas public schema, execution format, or active MVP status.

## Question

Wall-clock measurements are useful observations, but they are noisy, specific
to one environment, and poor explanations of why an algorithm behaves as it
does. Atlas needs a profile that can distinguish algorithmic work, locality,
control flow, generated instructions, and target-machine observations without
claiming that any one of these layers is the complete performance truth.

The desired result is not a universal score. It is a sourced, multidimensional
performance fingerprint whose components can be regenerated and compared at
the level where they were obtained.

## Main conclusion

Atlas should keep three performance layers separate:

1. **Portable semantic work** records operations and dynamic access/control
   sequences at the algorithm level.
2. **Parameterized models** derive locality, branch predictability and
   transfer estimates for explicitly defined abstract machines.
3. **Target observations** describe lowered instruction streams and hardware
   counters for an exact compiler, target, CPU and dataset.

No layer should silently supply missing facts to another. In particular:

- semantic reads are not necessarily retired load instructions;
- a sequential semantic access is not necessarily an L1 hit;
- a source comparison is not necessarily a machine branch;
- an instruction count is not a cycle estimate;
- a SIMD instruction is not automatically equivalent to its lane count in
  useful scalar work;
- a hardware counter is an observation, not a portable algorithm property.

## Why instruction counts need multiple cost dimensions

Different instructions have materially different execution behavior, but
assigning one fixed weight to each opcode would still be misleading. At least
the following dimensions must be distinguished:

- **latency**: cycles from an input becoming available to the result becoming
  available, relevant on a dependency chain;
- **reciprocal throughput**: average cycles per instruction for independent
  operations in steady state;
- **micro-operations and execution resources**: decoder, scheduler, execution
  port, load/store unit and register pressure;
- **critical path**: the longest dependency-constrained chain;
- **memory traffic**: bytes and cache lines transferred at each modeled level;
- **control speculation**: predicted and mispredicted control transfers under
  a named predictor or on a named processor;
- **vector utilization**: active lanes divided by available lanes, including
  masking and tail handling;
- **code size and front-end pressure**: encoded bytes, fetched lines and decoded
  operations.

LLVM exposes this distinction directly: its target cost interface has separate
cost kinds for reciprocal throughput, latency and code size, and treats a basic
addition and an expensive division as different cost classes. These are target
estimates used during lowering, not universal constants. See LLVM's
[TargetTransformInfo reference](https://llvm.org/doxygen/classllvm_1_1TargetTransformInfo.html).

The uops.info work gives an empirical methodology for measuring instruction
latency, throughput and execution-port usage independently rather than deriving
one number from elapsed time. See Abel and Reineke,
[uops.info](https://doi.org/10.1145/3297858.3304062) and its
[measurement-method summary](https://uops.info/paper.html).

### Instruction families to retain

The initial classification should remain small enough to explain and test:

| Family | Required distinctions | Principal risks |
|---|---|---|
| Integer add, subtract, logic and shifts | scalar/vector width, dependency chain, immediate folding | apparently free moves or address arithmetic may fold during lowering |
| Integer multiply | operand width, low/full-width result, dependency and execution resource | higher latency or lower throughput than simple integer operations |
| Integer divide and remainder | signed/unsigned, width, constant or variable divisor | commonly high and sometimes operand-sensitive latency; may lower to a sequence or helper call |
| Floating add, multiply and FMA | precision, scalar/vector width, contraction and fast-math policy | FMA changes both count and rounding; reductions expose dependency chains |
| Floating divide and square root | precision, scalar/vector form, exact or approximate operation | iterative or low-throughput units can dominate otherwise small kernels |
| Compare, select and branch | scalar/vector predicate, branchless lowering, outcome sequence | a semantic condition may become a branch, conditional move, mask or vector select |
| Load and store | width, alignment, address stream, cache/TLB level, read/write policy | latency spans cache hits, memory, translation and page faults; stores add ownership and eviction traffic |
| Gather, scatter and shuffle | lane width, active mask, address distribution | SIMD syntax can hide multiple memory transactions and expensive rearrangement |
| Conversion and packing | source/destination type, saturation and rounding | can require cross-domain transfers or multiple instructions |
| Call, allocation, atomic and fence | ABI, inlining, allocator, ordering and contention | effects cross the local instruction stream and may serialize execution |

This table is a taxonomy, not a weight table. Numerical latency, throughput and
resource fields are valid only with target provenance such as ISA, CPU model,
microcode, compiler, flags and generated instruction identity. For the current
host, AMD publishes a target-specific
[Zen 5 optimization guide](https://docs.amd.com/v/u/en-US/58455_1.00); it must
not be generalized to RV64 or another x86 microarchitecture.

### Why costs do not add linearly

For a dependency-bound loop, elapsed cycles tend toward its critical-path
latency. For independent operations, throughput and resource contention matter
more. Loads can overlap with arithmetic, multiple misses can be outstanding,
and out-of-order execution can hide some latency. Conversely, a single
loop-carried dependency or unpredictable branch may prevent that overlap.

Therefore Atlas should retain at least:

```text
instruction mix
+ dependency/critical-path structure
+ resource pressure
+ data-transfer model
+ control-prediction model
```

and should not reduce these components to `sum(count * nominal_cost)`.
`llvm-mca` is useful for target-specific throughput, dependencies and resource
pressure, but its own documentation states that accuracy depends on the quality
of the selected scheduling model. See the
[llvm-mca guide](https://llvm.org/docs/CommandGuide/llvm-mca.html).

## Memory locality model

The labels `sequential` and `random` are helpful summaries but insufficient
measurements. A dynamic address sequence should first be reduced to cache-line
identities for a declared line size, then analyzed using:

- bytes and unique cache lines read and written;
- signed stride histogram and contiguous-line run lengths;
- temporal reuse distance, including cold accesses;
- working-set or footprint curves over access windows;
- estimated miss-ratio curves for explicitly defined cache capacities and
  replacement assumptions;
- page and TLB working sets;
- requested versus useful bytes, especially for gathers, sparse structures and
  partial-line updates;
- memory-level parallelism opportunities where the dependency graph permits
  independent outstanding accesses.

Reuse distance is preferable to a binary sequential/random label because it
describes temporal locality independently of one cache capacity. Ding and Zhong
use it to characterize and predict whole-program locality:
[Predicting Whole-Program Locality through Reuse Distance Analysis](https://doi.org/10.1145/780822.781159).

For asymptotic transfers, Atlas can additionally report I/O complexity with
declared memory size `M` and block size `B`, following Aggarwal and Vitter's
[I/O model](https://doi.org/10.1145/48529.48535). Cache-oblivious analysis can
express bounds that hold across levels without selecting those parameters in
the algorithm, but still relies on explicit model assumptions; see Frigo et al.,
[Cache-Oblivious Algorithms](https://doi.org/10.1109/SFFCS.1999.814600).

Roofline relates operational intensity to compute and memory-bandwidth ceilings
and is valuable for regular numerical kernels:
[Williams, Waterman and Patterson](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2008/EECS-2008-134.html).
The ECM model further separates in-core execution from transfers through cache
levels: [Hofmann, Eitzinger and Fey](https://arxiv.org/abs/1509.03118). Neither
model alone captures irregular pointer chasing, branch-heavy code or short
non-steady-state algorithms, so they should be optional interpretations of the
fingerprint rather than the Atlas base model.

## Branch behavior

`Predictable` is not an intrinsic Boolean property of a branch. It depends on
its outcome history, correlations with other branches, predictor organization,
aliasing and warm state. The portable layer should retain, per semantic branch:

- execution count and taken/not-taken counts;
- transition count;
- run-length distribution;
- zero-order entropy;
- conditional entropy for explicitly bounded history lengths;
- early-exit position where applicable.

The parameterized layer can replay the outcome stream through named simple
predictors: always-taken, always-not-taken, one-bit, two-bit saturating and one
small two-level configuration. These are explanatory reference models, not
claims about a current proprietary predictor. Yeh and Patt demonstrate why
history and pattern organization materially change accuracy:
[Two-Level Adaptive Branch Prediction](https://doi.org/10.1145/123465.123475).

Linear branch entropy is a stronger architecture-independent characterization
than taken rate or transition rate alone and correlates with multiple predictor
families; see De Pestel, Eyerman and Eeckhout,
[Linear Branch Entropy](https://doi.org/10.1109/TC.2016.2601323).

The target layer may record retired branches and mispredictions. It must also
record the generated code because compilers can replace branches with selects,
masks, conditional moves or vector operations.

## Proposed Atlas performance fingerprint

The following is a conceptual internal record, not a proposed schema change:

```text
identity
  algorithm, implementation, dataset, input digest

semantic
  operation counts
  allocations, copies and requested auxiliary bytes
  branch outcome summaries
  memory reference stream summaries

parameterized
  cache-line and reuse-distance summaries for declared parameters
  block-transfer estimates for declared M and B
  reference-predictor results

lowered target
  source commit, compiler, flags, target and CPU model
  instruction-family mix and encoded bytes
  scalar/SIMD widths and useful-lane ratio
  llvm-mca throughput, critical sequence and resource pressure where supported

observed target
  retired instructions and cycles
  branches and branch misses
  cache/TLB events with exact event names
  time enabled/running, repetitions and normalization boundary
```

Every component needs Atlas provenance and a level such as `declared`,
`inferred`, `tested` or `observed`. A derived cache simulation is `inferred`
under its named model; a PMU value is `observed`; exact trace-to-AST agreement is
`tested`.

## Instrumentation boundaries

### Semantic execution

The AST or MIR interpreter is the best place to collect portable operations,
addresses and branch outcomes because instrumentation can be deterministic and
independent of host timing. Aggregation should be online: retain histograms and
summaries, not unbounded traces. Native Rust remains the correction authority,
and sampled executions must be checked against it.

### Generated code

The MIR observer and disassembler can classify the exact host or RV64
instruction stream. This layer must distinguish static instructions from their
dynamic execution counts. `llvm-mca` can analyze bounded basic blocks for the
current `znver5` host and for several installed RV64 scheduling models, but a
generic RV64 model is not evidence for a physical RV64 processor.

### Hardware counters

Linux `perf_event_open` can enable and disable counter groups immediately around
the measured kernel. Atlas must record raw event identity, grouping, exclusions,
and both `time_enabled` and `time_running`; unequal values mean multiplexing and
require an explicitly reported scale estimate. The interface defines retired
branch and misprediction events and the multiplexing fields:
[perf_event_open(2)](https://man7.org/linux/man-pages/man2/perf_event_open.2.html).

Cachegrind can provide repeatable cache and branch simulations, but its manual
warns that these models are basic and unlikely to reflect a modern machine. It
also uses virtual addresses and omits kernel and other-process effects. It is a
useful differential oracle, not target truth:
[Cachegrind manual](https://valgrind.org/docs/manual/cg-manual.html).

## Initial local probe

Environment observed on 2026-07-14:

- AMD Ryzen AI 9 HX 370, 24 logical CPUs, Zen 5 family;
- Linux `perf` 7.1.3 with `perf_event_paranoid=2`;
- LLVM/`llvm-mca` 22.1.8, with `znver5` and multiple RV64 processor models;
- Rust release benchmark binary already built from Atlas commit `4a697f8`.

The following feasibility probe was run around the complete adaptive benchmark
process, not around an isolated kernel:

```text
perf stat -e instructions,cycles,branches,branch-misses,cache-references,cache-misses \
  -- target/release/examples/compare_sorts sort.insertion.rust.slice.v1
```

It observed approximately 21.0 billion instructions, 3.45 billion cycles,
6.01 billion branches, 5.82 million branch misses, 2.74 million generic cache
references and 225 thousand generic cache misses. The benchmark also reported
41 scheduler migrations and rejected its timed series because of an extreme
sample.

These values prove tool availability only. They include dataset handling,
adaptive calibration, warmup, copies, observation, diagnostics and error
reporting. The generic cache events are additionally processor-defined and far
too few to represent semantic data accesses. No algorithm-performance
conclusion may be drawn from this probe.

## Reproducible experiment plan

### Experiment 1 - Isolated counter boundary

Add a private runner that prepares data before enabling a `perf_event_open`
group, repeats only the kernel while counters are enabled, consumes a checksum,
then disables and reads the group. Compare it with an empty-loop baseline.

Acceptance:

- exact dataset and implementation identity;
- correction checked outside the counted region;
- counter availability and multiplexing reported, never silently substituted;
- counts normalized per invocation with raw totals retained;
- setup, allocation and output-copy policies stated separately.

### Experiment 2 - Semantic locality and branches

Aggregate AST/MIR events online for:

- `is_sorted`: sorted, inversion at the start, inversion at the end;
- partition: none, all, alternating and seeded-uniform matches;
- insertion and merge: ascending, descending, few-unique and seeded-uniform;
- reverse: even and odd sizes as a regular streaming control case.

Use sizes derived from element width and the recorded L1, L2 and L3 capacities,
plus small pedagogical inputs. Compute stride, reuse-distance, footprint and
branch-history summaries without persisting full traces.

### Experiment 3 - Variable instruction costs

For selected inner loops, retain generated host and RV64 code and classify
dynamic instructions into the families above. Report:

- instruction count and bytes;
- latency-bound critical path estimate;
- reciprocal-throughput and execution-resource estimate;
- scalar versus SIMD work and useful-lane ratio;
- divide, square-root, gather/scatter, shuffle, atomic and fence occurrences as
  individually visible expensive classes;
- loads/stores separately from their modeled hierarchy cost.

Use `llvm-mca` only where a specific scheduling model is available. Validate a
small x86 subset against dependency-chain and independent-stream
microbenchmarks; do not transfer those numerical costs to RV64.

### Experiment 4 - Model versus target

Compare predicted cache/predictor trends with target PMU trends, not just their
absolute values. Expected questions include:

- Does alternating partition data increase reference-predictor and PMU branch
  misses relative to all/none cases at equal semantic work?
- Do merge's mostly sequential accesses show lower reuse-distance and cache-miss
  growth than a deliberately indirect access workload?
- Where do insertion datasets change operation count versus only instruction or
  prediction cost?
- Do SIMD reductions gain useful throughput after shuffle, mask and tail costs
  are included?

Disagreement is a result to explain, not a reason to tune the abstract model
until it reproduces one processor.

## Threats to validity

- Compiler optimizations can delete, combine, vectorize or speculate semantic
  operations.
- PMU event meanings and reliability vary by CPU and kernel.
- Hardware prefetching can make regular address streams cheaper without
  changing reuse distance.
- Cache associativity, physical indexing, TLBs and coherence are only partially
  represented by simple models.
- Out-of-order overlap prevents linear addition of nominal instruction and
  memory latencies.
- WASM, native Rust, MIR interpreter, host JIT and generated RV64 are different
  targets and must never share unlabeled results.
- Very small kernels are dominated by call, counter and loop overhead unless
  batched carefully.

## Recommended project sequence

1. Treat this report and the first private runner as research artifacts; do not
   modify schema 0.1.
2. Establish exact semantic address and branch streams for two contrasting
   algorithms before broadening the event vocabulary.
3. Add instruction-family and `llvm-mca` reports for one host and one RV64
   generated kernel.
4. Compare the model with isolated PMU observations across the accepted dataset
   matrix.
5. Only then propose whether a performance fingerprint belongs in a public
   schema, a derived report, or regenerable execution output.

The recommended strategic direction is a layered fingerprint. A single weighted
score would be easier to display, but would erase the distinction between
algorithm, compiler, microarchitecture and dataset that Atlas exists to make
explicit.
