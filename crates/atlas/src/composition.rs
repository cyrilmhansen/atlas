//! MVP 3's first deliberately narrow composition experiment.
//!
//! This is an internal, scenario-specific plan. It is not a registry entity or
//! a persistent format, and it does not attempt to enumerate arbitrary graphs.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CleanupComposition {
    pub id: &'static str,
    pub goal: &'static str,
    pub input: &'static str,
    pub output: &'static str,
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

/// Selects the first real MVP 3 pipeline for the declared-allocation objective.
pub fn cleanup_minimize_declared_allocations() -> CleanupComposition {
    CleanupComposition {
        id: "sequence.cleanup.experimental.v1",
        goal: "minimize declared intermediate allocations",
        input: "mutable Vec<i32>; predicate: i32 -> bool; order: ascending",
        output: "StableUniqueSequence<i32>",
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
pub fn cleanup_minimize_declared_expected_time() -> CleanupComposition {
    let allocation_plan = cleanup_minimize_declared_allocations();
    let mut selected = allocation_plan.rejected;
    selected.decision = "selected: filter is O(n), merge sort is O(n log n), and hash deduplication is declared O(n) expected for i32";
    let mut rejected = allocation_plan.selected;
    rejected.decision = "rejected: insertion sort and quadratic deduplication both have declared O(n^2) worst-case time";

    CleanupComposition {
        id: "sequence.cleanup.experimental.v1",
        goal: "minimize declared expected time",
        input: "mutable Vec<i32>; predicate: i32 -> bool; order: ascending; equality/hash: i32 implements Eq + Hash",
        output: "StableUniqueSequence<i32>",
        selected,
        rejected,
    }
}

pub fn render(composition: &CleanupComposition) -> String {
    let mut output = format!(
        "plan: {}\ngoal: {}\ninput: {}\noutput: {}\nselected:\n",
        composition.id, composition.goal, composition.input, composition.output
    );
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
        cleanup_minimize_declared_allocations, cleanup_minimize_declared_expected_time, render,
        render_rust_orchestration,
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
}
