# K-M4 independent dual-import source packet

Status: frozen for the experiment

Protocol: `k-m0.2`

Batch size: six subjects

## Experiment boundary

Two importers receive this exact packet. Each must work independently and must
not inspect:

- `registry/atlas.yaml`;
- `docs/phase2/k-m0-comparison.md`;
- `docs/phase2/k-m1-graph-corpus.md`;
- `docs/phase2/k-m2-dynamic-structures.md`;
- `docs/phase2/k-m3-streaming-approximation.md`;
- `docs/phase2/imports/importer-a/` or `importer-b/`;
- the K-M4 output directory of the other importer;
- Git history or diffs revealing those files.

Allowed local references:

- this packet;
- `docs/phase2/import-worksheet.md`;
- `docs/phase2/import-equivalence-rubric.md`;
- `docs/phase2/current-model-baseline.md`;
- `docs/schema-0.1.md`;
- `docs/vision.md` only when the preceding documents leave a term undefined.

Web access is allowed only for the frozen source locators below and links
directly reachable from those pages. Do not search for Atlas commentary,
existing Atlas imports or the other importer's work.

## Frozen subjects and sources

### 1. Binary-heap push

- subject: Rust `BinaryHeap::push`;
- release/API baseline: Rust 1.85.0;
- required source:
  <https://doc.rust-lang.org/1.85.0/std/collections/struct.BinaryHeap.html#method.push>;
- optional directly related source:
  <https://doc.rust-lang.org/1.85.0/std/collections/struct.BinaryHeap.html#time-complexity>.

### 2. Collision-aware hash-map insert

- subject: `hashbrown::HashMap::insert`;
- release: hashbrown 0.17.1;
- required source:
  <https://docs.rs/hashbrown/0.17.1/hashbrown/struct.HashMap.html#method.insert>;
- required crate context:
  <https://docs.rs/hashbrown/0.17.1/hashbrown/>;
- cost convention source:
  <https://doc.rust-lang.org/1.85.0/std/collections/index.html#performance>.

### 3. Online corrected moments

- subject: Welford online corrected sums of squares;
- source: B. P. Welford, *Note on a Method for Calculating Corrected Sums of
  Squares and Products*, Technometrics 4(3), 1962;
- DOI: <https://doi.org/10.1080/00401706.1962.10490022>.

### 4. Reservoir sampling Algorithm R

- subject: Vitter Algorithm R;
- source: Jeffrey S. Vitter, *Random Sampling with a Reservoir*, ACM TOMS
  11(1), 1985;
- DOI: <https://dl.acm.org/doi/10.1145/3147.3165>;
- readable author-paper copy:
  <https://dsf.berkeley.edu/cs286/papers/reservoirsampling-toms1985.pdf>.

### 5. Bloom method 2 membership

- subject: Bloom's bit-field method 2, including insertion and membership test;
- source: Burton H. Bloom, *Space/Time Trade-offs in Hash Coding with Allowable
  Errors*, Communications of the ACM 13(7), 1970;
- DOI: <https://dl.acm.org/doi/10.1145/362686.362692>;
- readable paper copy:
  <https://www.cs.princeton.edu/courses/archive/spr05/cos598E/bib/p422-bloom.pdf>.

### 6. Misra-Gries repeated elements

- subject: the bounded-counter repeated-elements algorithm for values occurring
  more than `n / k` times;
- source: J. Misra and David Gries, *Finding Repeated Elements*, Science of
  Computer Programming 2(2), 1982;
- DOI: <https://doi.org/10.1016/0167-6423(82)90012-0>;
- readable paper copy:
  <https://khoury.northeastern.edu/home/pandey/courses/cs7800/spring26/papers/mg.pdf>.

## Required deliverables

Create one Markdown worksheet per subject in the importer-specific output
directory, using these filenames:

- `binary-heap-push.md`;
- `hashbrown-insert.md`;
- `welford.md`;
- `reservoir-r.md`;
- `bloom.md`;
- `misra-gries.md`.

Each worksheet must contain:

1. start/end timestamps and source locators actually read;
2. bibliographic, algorithmic, representational and executable fidelity;
3. declared transformations;
4. proposed `Problem` identity and exact input/requires/output/ensures;
5. proposed `Algorithm` identity, requirements, determinism, time and memory;
6. implementation boundary and effects when the source defines one;
7. information that schema 0.1 cannot represent without lossy prose;
8. three concrete selection requests with accept/reject consequences;
9. unresolved ambiguity classified as source, protocol or model ambiguity;
10. explicit statement of whether any public schema change is requested.

Use source-bounded claims. Do not infer a general theorem from one executable
fixture. Do not force a value merely because schema 0.1 requires a field: record
the mismatch and propose the least-lossy temporary representation.

## Submission rule

After writing all six files, run whitespace checks only over the importer's own
output directory. Send the root agent a completion message listing the files
and the three most decision-relevant divergences. Do not read any newly created
peer or comparison file until the root agent confirms both submissions are
frozen.
