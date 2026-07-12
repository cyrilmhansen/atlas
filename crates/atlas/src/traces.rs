use core::cmp::Ordering;

use crate::{
    ast::{SemanticOperation, merge_sort_ast, partition_ast},
    datasets::GeneratedDataset,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraceEvent {
    Read {
        space: &'static str,
        index: usize,
        value: i32,
    },
    Write {
        space: &'static str,
        index: usize,
        value: i32,
    },
    Compare {
        left_index: usize,
        right_index: usize,
        ordering: Ordering,
    },
    Swap {
        left_index: usize,
        right_index: usize,
    },
    Recurse {
        start: usize,
        length: usize,
    },
    Allocate {
        space: &'static str,
        elements: usize,
    },
    Copy {
        from: &'static str,
        to: &'static str,
        start: usize,
        elements: usize,
    },
    Partition {
        start: usize,
        end: usize,
        boundary: usize,
    },
    Predicate {
        index: usize,
        value: i32,
        matched: bool,
    },
    Assert {
        invariant: &'static str,
        passed: bool,
    },
}

impl TraceEvent {
    pub fn operation(&self) -> SemanticOperation {
        match self {
            Self::Read { .. } => SemanticOperation::Read,
            Self::Write { .. } => SemanticOperation::Write,
            Self::Compare { .. } => SemanticOperation::Compare,
            Self::Swap { .. } => SemanticOperation::Swap,
            Self::Recurse { .. } => SemanticOperation::Recurse,
            Self::Allocate { .. } => SemanticOperation::Allocate,
            Self::Copy { .. } => SemanticOperation::Copy,
            Self::Partition { .. } => SemanticOperation::Partition,
            Self::Predicate { .. } => SemanticOperation::Predicate,
            Self::Assert { .. } => SemanticOperation::Assert,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticTrace {
    pub trace_version: &'static str,
    pub ast_id: &'static str,
    pub algorithm_id: &'static str,
    pub implementation_id: &'static str,
    pub dataset_spec_id: &'static str,
    pub dataset_case_id: &'static str,
    pub dataset_digest_sha256: String,
    pub steps: Vec<TraceStep>,
    pub result: Vec<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceStep {
    pub ast_node_id: &'static str,
    pub event: TraceEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceValidationError {
    pub step: Option<usize>,
    pub message: String,
}

pub fn validate_trace_against_ast(
    trace: &SemanticTrace,
    ast: &crate::ast::AlgorithmAst,
) -> Vec<TraceValidationError> {
    let mut errors = Vec::new();
    if trace.ast_id != ast.id {
        errors.push(TraceValidationError {
            step: None,
            message: format!("trace references AST {:?}, not {:?}", trace.ast_id, ast.id),
        });
    }
    for (index, step) in trace.steps.iter().enumerate() {
        match ast.operation_by_id(step.ast_node_id) {
            None => errors.push(TraceValidationError {
                step: Some(index),
                message: format!("unknown AST operation node {:?}", step.ast_node_id),
            }),
            Some(operation) if operation != step.event.operation() => {
                errors.push(TraceValidationError {
                    step: Some(index),
                    message: format!(
                        "AST node {:?} declares {operation:?}, but event is {:?}",
                        step.ast_node_id,
                        step.event.operation()
                    ),
                });
            }
            Some(_) => {}
        }
    }
    errors
}

fn push(steps: &mut Vec<TraceStep>, ast_node_id: &'static str, event: TraceEvent) {
    steps.push(TraceStep { ast_node_id, event });
}

pub fn trace_merge_sort(dataset: &GeneratedDataset) -> SemanticTrace {
    assert_eq!(dataset.problem_id, "sequence.sort");
    let original = dataset.values.clone();
    let mut values = original.clone();
    let mut scratch = vec![0; values.len()];
    let mut events = Vec::new();
    push(
        &mut events,
        "merge.allocate",
        TraceEvent::Allocate {
            space: "caller_scratch",
            elements: scratch.len(),
        },
    );
    merge_sort_recursive(&mut values, &mut scratch, 0, &mut events);
    push(
        &mut events,
        "merge.result.assert",
        TraceEvent::Assert {
            invariant: "result is sorted",
            passed: values.windows(2).all(|pair| pair[0] <= pair[1]),
        },
    );
    let mut permutation = values.clone();
    let mut expected = original;
    permutation.sort();
    expected.sort();
    push(
        &mut events,
        "merge.permutation.assert",
        TraceEvent::Assert {
            invariant: "result is a permutation of input",
            passed: permutation == expected,
        },
    );
    trace(
        merge_sort_ast().id,
        "sort.merge.top_down",
        "sort.merge_with_scratch.rust.slice.v1",
        dataset,
        events,
        values,
    )
}

fn merge_sort_recursive(
    values: &mut [i32],
    scratch: &mut [i32],
    offset: usize,
    events: &mut Vec<TraceStep>,
) {
    push(
        events,
        "merge.invoke",
        TraceEvent::Recurse {
            start: offset,
            length: values.len(),
        },
    );
    if values.len() < 2 {
        return;
    }

    let middle = values.len() / 2;
    {
        let (left, right) = values.split_at_mut(middle);
        let (left_scratch, right_scratch) = scratch.split_at_mut(middle);
        merge_sort_recursive(left, left_scratch, offset, events);
        merge_sort_recursive(right, right_scratch, offset + middle, events);
    }

    push(
        events,
        "merge.boundary.read",
        TraceEvent::Read {
            space: "values",
            index: offset + middle - 1,
            value: values[middle - 1],
        },
    );
    push(
        events,
        "merge.boundary.read",
        TraceEvent::Read {
            space: "values",
            index: offset + middle,
            value: values[middle],
        },
    );
    let boundary_ordering = values[middle - 1].cmp(&values[middle]);
    push(
        events,
        "merge.boundary.compare",
        TraceEvent::Compare {
            left_index: offset + middle - 1,
            right_index: offset + middle,
            ordering: boundary_ordering,
        },
    );
    if boundary_ordering != Ordering::Greater {
        push(
            events,
            "merge.ordered.assert",
            TraceEvent::Assert {
                invariant: "adjacent sorted runs already ordered",
                passed: true,
            },
        );
        return;
    }

    let (mut left, mut right) = (0, middle);
    for (output_index, output) in scratch.iter_mut().enumerate() {
        let take_right = if left == middle {
            true
        } else if right == values.len() {
            false
        } else {
            push(
                events,
                "merge.select.read",
                TraceEvent::Read {
                    space: "values",
                    index: offset + left,
                    value: values[left],
                },
            );
            push(
                events,
                "merge.select.read",
                TraceEvent::Read {
                    space: "values",
                    index: offset + right,
                    value: values[right],
                },
            );
            let ordering = values[right].cmp(&values[left]);
            push(
                events,
                "merge.select.compare",
                TraceEvent::Compare {
                    left_index: offset + right,
                    right_index: offset + left,
                    ordering,
                },
            );
            ordering == Ordering::Less
        };
        let source = if take_right {
            let source = right;
            right += 1;
            source
        } else {
            let source = left;
            left += 1;
            source
        };
        push(
            events,
            "merge.select.read",
            TraceEvent::Read {
                space: "values",
                index: offset + source,
                value: values[source],
            },
        );
        *output = values[source];
        push(
            events,
            "merge.output.write",
            TraceEvent::Write {
                space: "caller_scratch",
                index: offset + output_index,
                value: *output,
            },
        );
    }
    values.copy_from_slice(scratch);
    push(
        events,
        "merge.copy",
        TraceEvent::Copy {
            from: "caller_scratch",
            to: "values",
            start: offset,
            elements: values.len(),
        },
    );
    push(
        events,
        "merge.result.assert",
        TraceEvent::Assert {
            invariant: "merged range is sorted",
            passed: values.windows(2).all(|pair| pair[0] <= pair[1]),
        },
    );
}

pub fn trace_partition_in_place(dataset: &GeneratedDataset) -> SemanticTrace {
    assert_eq!(dataset.problem_id, "sequence.partition");
    let predicate = dataset
        .predicate
        .expect("partition dataset requires an explicit predicate");
    let original = dataset.values.clone();
    let mut values = original.clone();
    let (mut left, mut right) = (0, values.len());
    let mut events = Vec::new();

    while left < right {
        while left < right {
            let value = values[left];
            push(
                &mut events,
                "partition.left.read",
                TraceEvent::Read {
                    space: "values",
                    index: left,
                    value,
                },
            );
            let matched = predicate.matches(value);
            push(
                &mut events,
                "partition.left.predicate",
                TraceEvent::Predicate {
                    index: left,
                    value,
                    matched,
                },
            );
            if !matched {
                break;
            }
            left += 1;
        }
        while left < right {
            let index = right - 1;
            let value = values[index];
            push(
                &mut events,
                "partition.right.read",
                TraceEvent::Read {
                    space: "values",
                    index,
                    value,
                },
            );
            let matched = predicate.matches(value);
            push(
                &mut events,
                "partition.right.predicate",
                TraceEvent::Predicate {
                    index,
                    value,
                    matched,
                },
            );
            if matched {
                break;
            }
            right -= 1;
        }
        if left < right {
            values.swap(left, right - 1);
            push(
                &mut events,
                "partition.swap",
                TraceEvent::Swap {
                    left_index: left,
                    right_index: right - 1,
                },
            );
            left += 1;
            right -= 1;
        }
    }
    push(
        &mut events,
        "partition.boundary",
        TraceEvent::Partition {
            start: 0,
            end: values.len(),
            boundary: left,
        },
    );
    push(
        &mut events,
        "partition.left.assert",
        TraceEvent::Assert {
            invariant: "matching values precede boundary",
            passed: values[..left].iter().all(|value| predicate.matches(*value)),
        },
    );
    push(
        &mut events,
        "partition.right.assert",
        TraceEvent::Assert {
            invariant: "rejected values follow boundary",
            passed: values[left..]
                .iter()
                .all(|value| !predicate.matches(*value)),
        },
    );
    let mut permutation = values.clone();
    let mut expected = original;
    permutation.sort();
    expected.sort();
    push(
        &mut events,
        "partition.permutation.assert",
        TraceEvent::Assert {
            invariant: "result is a permutation of input",
            passed: permutation == expected,
        },
    );
    trace(
        partition_ast().id,
        "partition.two_pointer.in_place",
        "partition.in_place.rust.slice.v1",
        dataset,
        events,
        values,
    )
}

fn trace(
    ast_id: &'static str,
    algorithm_id: &'static str,
    implementation_id: &'static str,
    dataset: &GeneratedDataset,
    steps: Vec<TraceStep>,
    result: Vec<i32>,
) -> SemanticTrace {
    SemanticTrace {
        trace_version: "experimental-0",
        ast_id,
        algorithm_id,
        implementation_id,
        dataset_spec_id: dataset.spec_id,
        dataset_case_id: dataset.case_id,
        dataset_digest_sha256: dataset.content_digest_sha256.clone(),
        steps,
        result,
    }
}

#[cfg(test)]
mod tests {
    use atlas_algorithms::{merge_sort::merge_sort_by_with_scratch, partition::partition_in_place};

    use crate::{
        ast::{merge_sort_ast, partition_ast},
        datasets::{PARTITION_DATASET_SPEC, SORT_DATASET_SPEC},
    };

    use super::{
        TraceEvent, TraceStep, trace_merge_sort, trace_partition_in_place,
        validate_trace_against_ast,
    };

    #[test]
    fn merge_trace_is_deterministic_and_matches_native_result() {
        let dataset = SORT_DATASET_SPEC
            .generate(&SORT_DATASET_SPEC.cases[4])
            .unwrap();
        let first = trace_merge_sort(&dataset);
        let second = trace_merge_sort(&dataset);
        let mut native = dataset.values.clone();
        let mut scratch = native.clone();
        merge_sort_by_with_scratch(&mut native, &mut scratch, i32::cmp).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.result, native);
        assert!(
            first
                .steps
                .iter()
                .any(|step| matches!(step.event, TraceEvent::Recurse { .. }))
        );
        assert!(
            first
                .steps
                .iter()
                .any(|step| matches!(step.event, TraceEvent::Copy { .. }))
        );
        assert_all_invariants_pass(&first.steps);
        assert!(validate_trace_against_ast(&first, &merge_sort_ast()).is_empty());
    }

    #[test]
    fn partition_trace_is_deterministic_and_matches_native_result() {
        let dataset = PARTITION_DATASET_SPEC
            .generate(&PARTITION_DATASET_SPEC.cases[3])
            .unwrap();
        let first = trace_partition_in_place(&dataset);
        let second = trace_partition_in_place(&dataset);
        let predicate = dataset.predicate.unwrap();
        let mut native = dataset.values.clone();
        partition_in_place(&mut native, |value| predicate.matches(*value));

        assert_eq!(first, second);
        assert_eq!(first.result, native);
        assert!(
            first
                .steps
                .iter()
                .any(|step| matches!(step.event, TraceEvent::Swap { .. }))
        );
        assert!(
            first
                .steps
                .iter()
                .any(|step| matches!(step.event, TraceEvent::Partition { .. }))
        );
        assert_all_invariants_pass(&first.steps);
        assert!(validate_trace_against_ast(&first, &partition_ast()).is_empty());
    }

    #[test]
    fn exact_link_validation_rejects_unknown_and_mismatched_nodes() {
        let dataset = SORT_DATASET_SPEC
            .generate(&SORT_DATASET_SPEC.cases[4])
            .unwrap();
        let trace = trace_merge_sort(&dataset);

        let mut unknown = trace.clone();
        unknown.steps[0].ast_node_id = "merge.missing";
        let errors = validate_trace_against_ast(&unknown, &merge_sort_ast());
        assert_eq!(errors[0].step, Some(0));
        assert!(errors[0].message.contains("unknown AST operation node"));

        let mut mismatched = trace;
        mismatched.steps[0].ast_node_id = "merge.copy";
        let errors = validate_trace_against_ast(&mismatched, &merge_sort_ast());
        assert_eq!(errors[0].step, Some(0));
        assert!(
            errors[0]
                .message
                .contains("declares Copy, but event is Allocate")
        );
    }

    fn assert_all_invariants_pass(steps: &[TraceStep]) {
        assert!(
            steps
                .iter()
                .all(|step| !matches!(step.event, TraceEvent::Assert { passed: false, .. }))
        );
    }
}
