//! MVP 3's first deliberately narrow composition experiment.
//!
//! This is an internal, scenario-specific plan. It is not a registry entity or
//! a persistent format, and it does not attempt to enumerate arbitrary graphs.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Composition {
    pub id: &'static str,
    pub goal: &'static str,
    pub input: &'static str,
    pub output: &'static str,
    pub preconditions: &'static [&'static str],
    pub selected: CandidatePlan,
    pub rejected: CandidatePlan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidatePlan {
    pub id: &'static str,
    pub steps: Vec<PlanStep>,
    pub decision: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanStep {
    pub implementation_id: &'static str,
    pub input: &'static str,
    pub output: &'static str,
    pub effects: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImplementationConstraint<'a> {
    Force(&'a str),
    Forbid(&'a str),
}

/// Applies an explicit choice only within this scenario's reviewed candidates.
pub fn apply_implementation_constraint(
    mut composition: Composition,
    constraint: ImplementationConstraint<'_>,
) -> Result<Composition, String> {
    let selected_has = candidate_uses(&composition.selected, constraint.id());
    let rejected_has = candidate_uses(&composition.rejected, constraint.id());

    match constraint {
        ImplementationConstraint::Force(_) if selected_has => {
            composition.selected.decision = "selected: explicit force matches the normal candidate";
        }
        ImplementationConstraint::Force(_) if rejected_has => {
            std::mem::swap(&mut composition.selected, &mut composition.rejected);
            composition.selected.decision =
                "selected: explicit force overrides the normal candidate";
            composition.rejected.decision = "rejected: not selected by the explicit force";
        }
        ImplementationConstraint::Force(id) => {
            return Err(format!(
                "forced implementation {id:?} is not used by either reviewed candidate"
            ));
        }
        ImplementationConstraint::Forbid(id) if selected_has && rejected_has => {
            return Err(format!(
                "forbidden implementation {id:?} removes every reviewed candidate"
            ));
        }
        ImplementationConstraint::Forbid(_) if selected_has => {
            std::mem::swap(&mut composition.selected, &mut composition.rejected);
            composition.selected.decision =
                "selected: the normal candidate was excluded by an explicit forbid constraint";
            composition.rejected.decision = "rejected: excluded by the explicit forbid constraint";
        }
        ImplementationConstraint::Forbid(_) if rejected_has => {
            composition.selected.decision =
                "selected: the alternative was excluded by an explicit forbid constraint";
        }
        ImplementationConstraint::Forbid(id) => {
            return Err(format!(
                "forbidden implementation {id:?} is not used by either reviewed candidate"
            ));
        }
    }

    Ok(composition)
}

impl<'a> ImplementationConstraint<'a> {
    fn id(self) -> &'a str {
        match self {
            Self::Force(id) | Self::Forbid(id) => id,
        }
    }
}

fn candidate_uses(candidate: &CandidatePlan, implementation_id: &str) -> bool {
    candidate
        .steps
        .iter()
        .any(|step| step.implementation_id == implementation_id)
}

/// Selects the first real MVP 3 pipeline for the declared-allocation objective.
pub fn cleanup_minimize_declared_allocations() -> Composition {
    Composition {
        id: "sequence.cleanup.experimental.v1",
        goal: "minimize declared intermediate allocations",
        input: "mutable Vec<i32>; predicate: i32 -> bool; order: ascending",
        output: "StableUniqueSequence<i32>",
        preconditions: &[],
        selected: CandidatePlan {
            id: "cleanup.in_place_insertion_quadratic",
            steps: vec![
                PlanStep {
                    implementation_id: "filter.in_place.rust.vec.v1",
                    input: "input Vec<i32>",
                    output: "the same filtered Vec<i32>",
                    effects: &["mutates input.sequence", "allocation: none"],
                },
                PlanStep {
                    implementation_id: "sort.insertion.rust.slice.v1",
                    input: "the same filtered Vec<i32>",
                    output: "the same sorted Vec<i32>",
                    effects: &["mutates input.sequence", "allocation: none"],
                },
                PlanStep {
                    implementation_id: "deduplicate.quadratic.rust.vec.v1",
                    input: "the sorted Vec<i32>",
                    output: "new StableUniqueSequence<i32>",
                    effects: &[
                        "copies first occurrences into output",
                        "allocation: output Vec<T>",
                    ],
                },
            ],
            decision: "selected: filtering and sorting add no declared allocation; the required deduplicated output is the only declared allocation",
        },
        rejected: CandidatePlan {
            id: "cleanup.copy_merge_hash",
            steps: vec![
                PlanStep {
                    implementation_id: "filter.copy.rust.vec.v1",
                    input: "input Vec<i32>",
                    output: "new filtered Vec<i32>",
                    effects: &["copies matching values", "allocation: output Vec<T>"],
                },
                PlanStep {
                    implementation_id: "sort.merge.rust.slice.v1",
                    input: "the filtered Vec<i32>",
                    output: "the same sorted Vec<i32>",
                    effects: &["mutates input.sequence", "allocation: auxiliary Vec<T>"],
                },
                PlanStep {
                    implementation_id: "deduplicate.hash.rust.vec.v1",
                    input: "the sorted Vec<i32>",
                    output: "new StableUniqueSequence<i32>",
                    effects: &[
                        "copies first occurrences into output",
                        "allocation: output Vec<T> and internal HashSet<T>",
                    ],
                },
            ],
            decision: "rejected: it introduces a copied filter result, merge scratch storage, and hash-set storage before the required output",
        },
    }
}

/// Selects the same pipeline for declared expected time rather than allocation.
///
/// This is a statement about registry complexity claims. It is not a benchmark
/// result and does not claim a universal latency ordering.
pub fn cleanup_minimize_declared_expected_time() -> Composition {
    let allocation_plan = cleanup_minimize_declared_allocations();
    let mut selected = allocation_plan.rejected;
    selected.decision = "selected: filter is O(n), merge sort is O(n log n), and hash deduplication is declared O(n) expected for i32";
    let mut rejected = allocation_plan.selected;
    rejected.decision = "rejected: insertion sort and quadratic deduplication both have declared O(n^2) worst-case time";

    Composition {
        id: "sequence.cleanup.experimental.v1",
        goal: "minimize declared expected time",
        input: "mutable Vec<i32>; predicate: i32 -> bool; order: ascending; equality/hash: i32 implements Eq + Hash",
        output: "StableUniqueSequence<i32>",
        preconditions: &[],
        selected,
        rejected,
    }
}

/// Selects a sorting/searching composition that establishes binary search's
/// sorted-input precondition without a declared intermediate allocation.
pub fn find_minimize_declared_allocations() -> Composition {
    Composition {
        id: "sequence.find.experimental.v1",
        goal: "satisfy sorted-input precondition with no declared intermediate allocation",
        input: "mutable Vec<i32>; needle: i32; order: ascending",
        output: "Option<Index<input.sequence>>",
        preconditions: &[
            "binary search requires input.sequence sorted according to the comparison order",
            "step 1 establishes the binary-search precondition before step 2",
        ],
        selected: CandidatePlan {
            id: "find.insertion_binary",
            steps: vec![
                PlanStep {
                    implementation_id: "sort.insertion.rust.slice.v1",
                    input: "input Vec<i32>",
                    output: "the same sorted Vec<i32>",
                    effects: &["mutates input.sequence", "allocation: none"],
                },
                PlanStep {
                    implementation_id: "search.binary.rust.slice.v1",
                    input: "the sorted Vec<i32> and needle",
                    output: "Option<Index<input.sequence>>",
                    effects: &["reads input.sequence", "allocation: none"],
                },
            ],
            decision: "selected: insertion sorting establishes the required order without a declared intermediate allocation",
        },
        rejected: CandidatePlan {
            id: "find.merge_binary",
            steps: vec![
                PlanStep {
                    implementation_id: "sort.merge.rust.slice.v1",
                    input: "input Vec<i32>",
                    output: "the same sorted Vec<i32>",
                    effects: &["mutates input.sequence", "allocation: auxiliary Vec<T>"],
                },
                PlanStep {
                    implementation_id: "search.binary.rust.slice.v1",
                    input: "the sorted Vec<i32> and needle",
                    output: "Option<Index<input.sequence>>",
                    effects: &["reads input.sequence", "allocation: none"],
                },
            ],
            decision: "rejected: it establishes the same precondition but declares merge-sort scratch storage",
        },
    }
}

/// Composes sorting with deduplication while making the required output
/// allocation distinct from intermediate storage.
pub fn unique_sort_minimize_declared_allocations() -> Composition {
    Composition {
        id: "sequence.unique_sort.experimental.v1",
        goal: "minimize declared intermediate allocations while producing sorted unique output",
        input: "mutable Sequence<i32>; order: ascending",
        output: "StableUniqueSequence<i32>",
        preconditions: &[],
        selected: CandidatePlan {
            id: "unique_sort.insertion_quadratic",
            steps: vec![
                PlanStep {
                    implementation_id: "sort.insertion.rust.slice.v1",
                    input: "input mutable Sequence<i32>",
                    output: "the same sorted Sequence<i32>",
                    effects: &["mutates input.sequence", "allocation: none"],
                },
                PlanStep {
                    implementation_id: "deduplicate.quadratic.rust.vec.v1",
                    input: "the sorted Sequence<i32>",
                    output: "new StableUniqueSequence<i32>",
                    effects: &[
                        "copies first occurrences into output",
                        "allocation: output Vec<T>",
                    ],
                },
            ],
            decision: "selected: sorting adds no declared allocation; the required unique output is the only declared allocation",
        },
        rejected: CandidatePlan {
            id: "unique_sort.merge_hash",
            steps: vec![
                PlanStep {
                    implementation_id: "sort.merge.rust.slice.v1",
                    input: "input mutable Sequence<i32>",
                    output: "the same sorted Sequence<i32>",
                    effects: &["mutates input.sequence", "allocation: auxiliary Vec<T>"],
                },
                PlanStep {
                    implementation_id: "deduplicate.hash.rust.vec.v1",
                    input: "the sorted Sequence<i32>",
                    output: "new StableUniqueSequence<i32>",
                    effects: &[
                        "copies first occurrences into output",
                        "allocation: output Vec<T> and internal HashSet<T>",
                    ],
                },
            ],
            decision: "rejected: merge scratch storage and hash-set storage are additional declared allocations before the required output",
        },
    }
}

/// Composes two independent sorts with a stable two-way merge.
pub fn merge_sorted_minimize_declared_allocations() -> Composition {
    Composition {
        id: "sequence.merge_sorted.experimental.v1",
        goal: "establish two sorted-input preconditions with no declared intermediate allocation",
        input: "left and right mutable Sequence<i32>; shared order: ascending",
        output: "StableMergedSequence<i32>",
        preconditions: &[
            "two-way merge requires left and right sorted according to the same comparison order",
            "steps 1 and 2 establish the merge preconditions before step 3",
        ],
        selected: CandidatePlan {
            id: "merge_sorted.insertion_insertion_merge",
            steps: vec![
                PlanStep {
                    implementation_id: "sort.insertion.rust.slice.v1",
                    input: "left mutable Sequence<i32>",
                    output: "the same sorted left Sequence<i32>",
                    effects: &["mutates left.sequence", "allocation: none"],
                },
                PlanStep {
                    implementation_id: "sort.insertion.rust.slice.v1",
                    input: "right mutable Sequence<i32>",
                    output: "the same sorted right Sequence<i32>",
                    effects: &["mutates right.sequence", "allocation: none"],
                },
                PlanStep {
                    implementation_id: "merge.sorted.rust.vec.v1",
                    input: "the sorted left and right sequences",
                    output: "new StableMergedSequence<i32>",
                    effects: &[
                        "reads both sorted input sequences",
                        "allocation: merged output Vec<T>",
                    ],
                },
            ],
            decision: "selected: both prerequisite sorts add no declared allocation; the merged output is the only declared allocation",
        },
        rejected: CandidatePlan {
            id: "merge_sorted.merge_merge_merge",
            steps: vec![
                PlanStep {
                    implementation_id: "sort.merge.rust.slice.v1",
                    input: "left mutable Sequence<i32>",
                    output: "the same sorted left Sequence<i32>",
                    effects: &["mutates left.sequence", "allocation: auxiliary Vec<T>"],
                },
                PlanStep {
                    implementation_id: "sort.merge.rust.slice.v1",
                    input: "right mutable Sequence<i32>",
                    output: "the same sorted right Sequence<i32>",
                    effects: &["mutates right.sequence", "allocation: auxiliary Vec<T>"],
                },
                PlanStep {
                    implementation_id: "merge.sorted.rust.vec.v1",
                    input: "the sorted left and right sequences",
                    output: "new StableMergedSequence<i32>",
                    effects: &[
                        "reads both sorted input sequences",
                        "allocation: merged output Vec<T>",
                    ],
                },
            ],
            decision: "rejected: it establishes the same two preconditions but declares auxiliary merge-sort storage for both inputs",
        },
    }
}

/// Composes a stable partition with sorting of its matching branch.
pub fn partition_sort_minimize_declared_allocations() -> Composition {
    Composition {
        id: "sequence.partition_sort.experimental.v1",
        goal: "sort the matching partition with no declared allocation beyond partition outputs",
        input: "Sequence<i32>; predicate: i32 -> bool; order: ascending",
        output: "Partition { matching: SortedSequence<i32>, rejected: StableSubsequence<i32> }",
        preconditions: &[
            "project partition.matching, sort it, then retain partition.rejected in the result",
        ],
        selected: CandidatePlan {
            id: "partition_sort.copy_insertion",
            steps: vec![
                PlanStep {
                    implementation_id: "partition.copy.rust.vec.v1",
                    input: "input sequence and predicate",
                    output: "Partition { matching: Vec<i32>, rejected: Vec<i32> }",
                    effects: &["allocates matching and rejected output Vec<T> values"],
                },
                PlanStep {
                    implementation_id: "projection.partition.matching",
                    input: "partition.matching",
                    output: "borrowed matching Vec<i32>",
                    effects: &[
                        "projects one branch; no copy or allocation",
                        "retains partition.rejected",
                    ],
                },
                PlanStep {
                    implementation_id: "sort.insertion.rust.slice.v1",
                    input: "borrowed matching Vec<i32>",
                    output: "the same sorted matching Vec<i32>",
                    effects: &["mutates partition.matching", "allocation: none"],
                },
                PlanStep {
                    implementation_id: "reassemble.partition",
                    input: "sorted matching and retained rejected branch",
                    output: "Partition { matching: SortedSequence<i32>, rejected: StableSubsequence<i32> }",
                    effects: &["moves both branches into result; no copy or allocation"],
                },
            ],
            decision: "selected: insertion sort adds no declared allocation after the required partition outputs",
        },
        rejected: CandidatePlan {
            id: "partition_sort.copy_merge",
            steps: vec![
                PlanStep {
                    implementation_id: "partition.copy.rust.vec.v1",
                    input: "input sequence and predicate",
                    output: "Partition { matching: Vec<i32>, rejected: Vec<i32> }",
                    effects: &["allocates matching and rejected output Vec<T> values"],
                },
                PlanStep {
                    implementation_id: "projection.partition.matching",
                    input: "partition.matching",
                    output: "borrowed matching Vec<i32>",
                    effects: &[
                        "projects one branch; no copy or allocation",
                        "retains partition.rejected",
                    ],
                },
                PlanStep {
                    implementation_id: "sort.merge.rust.slice.v1",
                    input: "borrowed matching Vec<i32>",
                    output: "the same sorted matching Vec<i32>",
                    effects: &["mutates partition.matching", "allocation: auxiliary Vec<T>"],
                },
                PlanStep {
                    implementation_id: "reassemble.partition",
                    input: "sorted matching and retained rejected branch",
                    output: "Partition { matching: SortedSequence<i32>, rejected: StableSubsequence<i32> }",
                    effects: &["moves both branches into result; no copy or allocation"],
                },
            ],
            decision: "rejected: merge sort adds declared scratch storage after the same projection",
        },
    }
}

pub fn render(composition: &Composition) -> String {
    let mut output = format!(
        "plan: {}\ngoal: {}\ninput: {}\noutput: {}\n",
        composition.id, composition.goal, composition.input, composition.output
    );
    if !composition.preconditions.is_empty() {
        output.push_str("preconditions:\n");
        for precondition in composition.preconditions {
            output.push_str(&format!("  - {precondition}\n"));
        }
    }
    output.push_str("selected:\n");
    render_candidate(&mut output, &composition.selected);
    output.push_str("rejected:\n");
    render_candidate(&mut output, &composition.rejected);
    output
}

/// Returns the verified Rust orchestration source for the selected candidate.
///
/// The source is compiled as the `cleanup_generated` Cargo example. It remains
/// a generated display product, not a stored plan format.
pub fn render_rust_orchestration() -> &'static str {
    include_str!("../examples/cleanup_generated.rs")
}

/// Returns the verified Rust source for the expected-time selected candidate.
pub fn render_expected_time_rust_orchestration() -> &'static str {
    include_str!("../examples/cleanup_expected_time_generated.rs")
}

/// Returns the verified Rust source for the precondition-focused candidate.
pub fn render_find_rust_orchestration() -> &'static str {
    include_str!("../examples/find_generated.rs")
}

/// Returns the verified Rust source for the sorted-unique candidate.
pub fn render_unique_sort_rust_orchestration() -> &'static str {
    include_str!("../examples/unique_sort_generated.rs")
}

/// Returns the verified Rust source for the two-input merge candidate.
pub fn render_merge_sorted_rust_orchestration() -> &'static str {
    include_str!("../examples/merge_sorted_generated.rs")
}

pub fn render_partition_sort_rust_orchestration() -> &'static str {
    include_str!("../examples/partition_sort_generated.rs")
}

fn render_candidate(output: &mut String, candidate: &CandidatePlan) {
    output.push_str(&format!("  id: {}\n", candidate.id));
    for (index, step) in candidate.steps.iter().enumerate() {
        output.push_str(&format!(
            "  step {}: {}\n    input: {}\n    output: {}\n",
            index + 1,
            step.implementation_id,
            step.input,
            step.output
        ));
        for effect in step.effects {
            output.push_str(&format!("    effect: {effect}\n"));
        }
    }
    output.push_str(&format!("  decision: {}\n", candidate.decision));
}

#[cfg(test)]
mod tests {
    use super::{
        ImplementationConstraint, apply_implementation_constraint,
        cleanup_minimize_declared_allocations, cleanup_minimize_declared_expected_time,
        find_minimize_declared_allocations, merge_sorted_minimize_declared_allocations,
        partition_sort_minimize_declared_allocations, render,
        render_expected_time_rust_orchestration, render_find_rust_orchestration,
        render_merge_sorted_rust_orchestration, render_rust_orchestration,
        render_unique_sort_rust_orchestration, unique_sort_minimize_declared_allocations,
    };

    #[test]
    fn selected_cleanup_plan_makes_mutations_copies_and_allocations_visible() {
        let composition = cleanup_minimize_declared_allocations();

        assert_eq!(composition.selected.steps.len(), 3);
        assert_eq!(
            composition.selected.steps[0].implementation_id,
            "filter.in_place.rust.vec.v1"
        );
        assert!(
            composition.selected.steps[0]
                .effects
                .contains(&"mutates input.sequence")
        );
        assert!(
            composition.selected.steps[2]
                .effects
                .contains(&"copies first occurrences into output")
        );
        assert!(
            composition.selected.steps[2]
                .effects
                .contains(&"allocation: output Vec<T>")
        );
    }

    #[test]
    fn rendered_plan_explains_the_rejected_allocation_heavier_alternative() {
        let output = render(&cleanup_minimize_declared_allocations());

        assert!(output.contains("rejected:\n  id: cleanup.copy_merge_hash"));
        assert!(output.contains("allocation: auxiliary Vec<T>"));
        assert!(output.contains("internal HashSet<T>"));
        assert!(output.contains("rejected: it introduces a copied filter result"));
    }

    #[test]
    fn generated_rust_orchestration_matches_the_selected_operations() {
        let source = render_rust_orchestration();

        assert!(source.contains("filter_in_place(values, predicate)"));
        assert!(source.contains("insertion_sort_by(values, i32::cmp)"));
        assert!(source.contains("deduplicate_quadratic(values)"));
    }

    #[test]
    fn expected_time_objective_selects_the_other_compatible_candidate() {
        let composition = cleanup_minimize_declared_expected_time();

        assert_eq!(composition.goal, "minimize declared expected time");
        assert_eq!(composition.selected.id, "cleanup.copy_merge_hash");
        assert!(composition.input.contains("Eq + Hash"));
        assert!(composition.selected.decision.contains("O(n log n)"));
        assert!(composition.rejected.decision.contains("O(n^2)"));
    }

    #[test]
    fn generated_expected_time_rust_orchestration_matches_the_selected_operations() {
        let source = render_expected_time_rust_orchestration();

        assert!(source.contains("filter_copy(values, predicate)"));
        assert!(source.contains("merge_sort_by(&mut filtered, i32::cmp)"));
        assert!(source.contains("deduplicate_hash(&filtered)"));
    }

    #[test]
    fn find_plan_exposes_and_establishes_binary_search_precondition() {
        let composition = find_minimize_declared_allocations();
        let output = render(&composition);

        assert_eq!(composition.selected.id, "find.insertion_binary");
        assert!(output.contains("preconditions:"));
        assert!(output.contains("step 1 establishes the binary-search precondition"));
        assert!(output.contains("search.binary.rust.slice.v1"));
        assert!(output.contains("rejected: it establishes the same precondition"));
    }

    #[test]
    fn generated_find_rust_orchestration_matches_the_selected_operations() {
        let source = render_find_rust_orchestration();

        assert!(source.contains("insertion_sort_by(values, i32::cmp)"));
        assert!(source.contains("binary_search_by(values, needle, i32::cmp)"));
    }

    #[test]
    fn unique_sort_plan_separates_required_output_from_intermediate_allocations() {
        let composition = unique_sort_minimize_declared_allocations();
        let output = render(&composition);

        assert_eq!(composition.selected.id, "unique_sort.insertion_quadratic");
        assert_eq!(composition.selected.steps.len(), 2);
        assert!(
            composition.selected.steps[0]
                .effects
                .contains(&"allocation: none")
        );
        assert!(
            composition.selected.steps[1]
                .effects
                .contains(&"allocation: output Vec<T>")
        );
        assert!(output.contains("rejected:\n  id: unique_sort.merge_hash"));
        assert!(output.contains("internal HashSet<T>"));
    }

    #[test]
    fn generated_unique_sort_rust_orchestration_matches_the_selected_operations() {
        let source = render_unique_sort_rust_orchestration();

        assert!(source.contains("insertion_sort_by(values, i32::cmp)"));
        assert!(source.contains("deduplicate_quadratic(values)"));
    }

    #[test]
    fn merge_sorted_plan_establishes_both_sorted_input_preconditions() {
        let composition = merge_sorted_minimize_declared_allocations();
        let output = render(&composition);

        assert_eq!(
            composition.selected.id,
            "merge_sorted.insertion_insertion_merge"
        );
        assert_eq!(composition.selected.steps.len(), 3);
        assert!(output.contains("steps 1 and 2 establish the merge preconditions"));
        assert!(output.contains("mutates left.sequence"));
        assert!(output.contains("mutates right.sequence"));
        assert!(output.contains("rejected:\n  id: merge_sorted.merge_merge_merge"));
        assert!(output.contains("auxiliary merge-sort storage for both inputs"));
    }

    #[test]
    fn generated_merge_sorted_rust_orchestration_matches_the_selected_operations() {
        let source = render_merge_sorted_rust_orchestration();

        assert!(source.contains("insertion_sort_by(left, i32::cmp)"));
        assert!(source.contains("insertion_sort_by(right, i32::cmp)"));
        assert!(source.contains("merge_sorted_values(left, right, i32::cmp)"));
    }

    #[test]
    fn partition_sort_plan_keeps_the_non_matching_branch_visible() {
        let composition = partition_sort_minimize_declared_allocations();
        let output = render(&composition);

        assert_eq!(composition.selected.id, "partition_sort.copy_insertion");
        assert!(output.contains("projection.partition.matching"));
        assert!(output.contains("retains partition.rejected"));
        assert!(output.contains("reassemble.partition"));
    }

    #[test]
    fn force_and_forbid_select_only_the_reviewed_cleanup_candidates() {
        let forced = apply_implementation_constraint(
            cleanup_minimize_declared_allocations(),
            ImplementationConstraint::Force("sort.merge.rust.slice.v1"),
        )
        .expect("merge sort belongs to the alternative");
        assert_eq!(forced.selected.id, "cleanup.copy_merge_hash");
        assert!(forced.selected.decision.contains("explicit force"));

        let forbidden = apply_implementation_constraint(
            cleanup_minimize_declared_allocations(),
            ImplementationConstraint::Forbid("filter.in_place.rust.vec.v1"),
        )
        .expect("the alternative remains available");
        assert_eq!(forbidden.selected.id, "cleanup.copy_merge_hash");
        assert!(forbidden.selected.decision.contains("explicit forbid"));
    }

    #[test]
    fn constraint_rejects_unknown_or_shared_required_implementations() {
        let unknown = apply_implementation_constraint(
            cleanup_minimize_declared_allocations(),
            ImplementationConstraint::Force("reverse.symmetric.rust.slice.v1"),
        )
        .expect_err("unknown candidate implementation must fail");
        assert!(unknown.contains("not used"));

        let shared = apply_implementation_constraint(
            find_minimize_declared_allocations(),
            ImplementationConstraint::Forbid("search.binary.rust.slice.v1"),
        )
        .expect_err("both find candidates require binary search");
        assert!(shared.contains("removes every"));
    }
}
