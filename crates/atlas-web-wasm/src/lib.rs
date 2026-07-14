use core::cmp::Ordering;

use atlas_algorithms::insertion_sort::insertion_sort_by;
use atlas_algorithms::is_sorted::is_sorted_by;
use atlas_algorithms::reverse::reverse_in_place;
use wasm_bindgen::prelude::*;

pub const MAX_INPUT_LENGTH: usize = 4_096;
pub const MAX_TRACE_INPUT_LENGTH: usize = 64;

const LEFT_READ_NODE: &str = "is-sorted.left.read";
const RIGHT_READ_NODE: &str = "is-sorted.right.read";
const COMPARE_NODE: &str = "is-sorted.adjacent.compare";

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IsSortedObservation {
    sorted: bool,
    comparisons: u32,
    first_inversion: Option<u32>,
}

#[wasm_bindgen]
impl IsSortedObservation {
    #[wasm_bindgen(getter)]
    pub fn sorted(&self) -> bool {
        self.sorted
    }

    #[wasm_bindgen(getter)]
    pub fn comparisons(&self) -> u32 {
        self.comparisons
    }

    #[wasm_bindgen(getter)]
    pub fn first_inversion(&self) -> Option<u32> {
        self.first_inversion
    }
}

#[wasm_bindgen]
pub fn observe_is_sorted_i32(values: &[i32]) -> Result<IsSortedObservation, JsError> {
    observe_is_sorted(values).map_err(|length| {
        JsError::new(&format!(
            "input length {length} exceeds the Atlas browser limit of {MAX_INPUT_LENGTH}"
        ))
    })
}

pub fn observe_is_sorted(values: &[i32]) -> Result<IsSortedObservation, usize> {
    if values.len() > MAX_INPUT_LENGTH {
        return Err(values.len());
    }

    let mut comparisons = 0_u32;
    let mut first_inversion = None;
    let sorted = is_sorted_by(values, |left, right| {
        comparisons += 1;
        let ordering = left.cmp(right);
        if ordering == Ordering::Greater {
            first_inversion = Some(comparisons);
        }
        ordering
    });

    Ok(IsSortedObservation {
        sorted,
        comparisons,
        first_inversion,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TraceOperation {
    Read,
    Compare,
}

impl TraceOperation {
    fn name(self) -> &'static str {
        match self {
            Self::Read => "Read",
            Self::Compare => "Compare",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct IsSortedTraceEvent {
    node_id: &'static str,
    operation: TraceOperation,
    left_index: u32,
    right_index: Option<u32>,
    ordering: Option<i8>,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IsSortedTrace {
    sorted: bool,
    first_inversion: Option<u32>,
    events: Vec<IsSortedTraceEvent>,
}

#[wasm_bindgen]
impl IsSortedTrace {
    #[wasm_bindgen(getter)]
    pub fn sorted(&self) -> bool {
        self.sorted
    }

    #[wasm_bindgen(getter)]
    pub fn first_inversion(&self) -> Option<u32> {
        self.first_inversion
    }

    #[wasm_bindgen(getter)]
    pub fn event_count(&self) -> u32 {
        self.events.len() as u32
    }

    pub fn event_node_id(&self, index: u32) -> Option<String> {
        self.events
            .get(index as usize)
            .map(|event| event.node_id.to_owned())
    }

    pub fn event_operation(&self, index: u32) -> Option<String> {
        self.events
            .get(index as usize)
            .map(|event| event.operation.name().to_owned())
    }

    pub fn event_left_index(&self, index: u32) -> Option<u32> {
        self.events
            .get(index as usize)
            .map(|event| event.left_index)
    }

    pub fn event_right_index(&self, index: u32) -> Option<u32> {
        self.events
            .get(index as usize)
            .and_then(|event| event.right_index)
    }

    pub fn event_ordering(&self, index: u32) -> Option<i8> {
        self.events
            .get(index as usize)
            .and_then(|event| event.ordering)
    }
}

#[wasm_bindgen]
pub fn trace_is_sorted_i32(values: &[i32]) -> Result<IsSortedTrace, JsError> {
    trace_is_sorted(values).map_err(|length| {
        JsError::new(&format!(
            "trace input length {length} exceeds the Atlas Explore limit of {MAX_TRACE_INPUT_LENGTH}"
        ))
    })
}

pub fn trace_is_sorted(values: &[i32]) -> Result<IsSortedTrace, usize> {
    if values.len() > MAX_TRACE_INPUT_LENGTH {
        return Err(values.len());
    }

    let mut events = Vec::with_capacity(values.len().saturating_sub(1) * 3);
    let mut first_inversion = None;
    let sorted = is_sorted_by(values, |left, right| {
        let right_index = (events.len() / 3 + 1) as u32;
        let left_index = right_index - 1;
        events.push(IsSortedTraceEvent {
            node_id: LEFT_READ_NODE,
            operation: TraceOperation::Read,
            left_index,
            right_index: None,
            ordering: None,
        });
        events.push(IsSortedTraceEvent {
            node_id: RIGHT_READ_NODE,
            operation: TraceOperation::Read,
            left_index: right_index,
            right_index: None,
            ordering: None,
        });
        let ordering = left.cmp(right);
        events.push(IsSortedTraceEvent {
            node_id: COMPARE_NODE,
            operation: TraceOperation::Compare,
            left_index,
            right_index: Some(right_index),
            ordering: Some(match ordering {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }),
        });
        if ordering == Ordering::Greater {
            first_inversion = Some(right_index);
        }
        ordering
    });

    Ok(IsSortedTrace {
        sorted,
        first_inversion,
        events,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TaggedValue {
    value: i32,
    original_index: u32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsertionSortObservation {
    values: Vec<i32>,
    original_indices: Vec<u32>,
    comparisons: u32,
    swaps: u32,
}

#[wasm_bindgen]
impl InsertionSortObservation {
    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Box<[i32]> {
        self.values.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn original_indices(&self) -> Box<[u32]> {
        self.original_indices.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn comparisons(&self) -> u32 {
        self.comparisons
    }

    #[wasm_bindgen(getter)]
    pub fn swaps(&self) -> u32 {
        self.swaps
    }
}

#[wasm_bindgen]
pub fn observe_insertion_sort_i32(values: &[i32]) -> Result<InsertionSortObservation, JsError> {
    observe_insertion_sort(values).map_err(|length| {
        JsError::new(&format!(
            "input length {length} exceeds the Atlas browser limit of {MAX_INPUT_LENGTH}"
        ))
    })
}

pub fn observe_insertion_sort(values: &[i32]) -> Result<InsertionSortObservation, usize> {
    if values.len() > MAX_INPUT_LENGTH {
        return Err(values.len());
    }

    let mut tagged: Vec<_> = values
        .iter()
        .copied()
        .enumerate()
        .map(|(original_index, value)| TaggedValue {
            value,
            original_index: original_index as u32,
        })
        .collect();
    let mut comparisons = 0_u32;
    let mut swaps = 0_u32;
    insertion_sort_by(&mut tagged, |left, right| {
        comparisons += 1;
        let ordering = left.value.cmp(&right.value);
        if ordering == Ordering::Less {
            swaps += 1;
        }
        ordering
    });

    Ok(InsertionSortObservation {
        values: tagged.iter().map(|item| item.value).collect(),
        original_indices: tagged.iter().map(|item| item.original_index).collect(),
        comparisons,
        swaps,
    })
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReverseObservation {
    values: Vec<i32>,
    reads: u32,
    writes: u32,
    swaps: u32,
}

#[wasm_bindgen]
impl ReverseObservation {
    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Box<[i32]> {
        self.values.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn reads(&self) -> u32 {
        self.reads
    }

    #[wasm_bindgen(getter)]
    pub fn writes(&self) -> u32 {
        self.writes
    }

    #[wasm_bindgen(getter)]
    pub fn swaps(&self) -> u32 {
        self.swaps
    }
}

#[wasm_bindgen]
pub fn observe_reverse_i32(values: &[i32]) -> Result<ReverseObservation, JsError> {
    observe_reverse(values).map_err(|length| {
        JsError::new(&format!(
            "input length {length} exceeds the Atlas browser limit of {MAX_INPUT_LENGTH}"
        ))
    })
}

pub fn observe_reverse(values: &[i32]) -> Result<ReverseObservation, usize> {
    if values.len() > MAX_INPUT_LENGTH {
        return Err(values.len());
    }

    let mut output = values.to_vec();
    reverse_in_place(&mut output);
    let swaps = (values.len() / 2) as u32;

    Ok(ReverseObservation {
        values: output,
        reads: swaps * 2,
        writes: swaps * 2,
        swaps,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        COMPARE_NODE, InsertionSortObservation, IsSortedObservation, LEFT_READ_NODE,
        MAX_INPUT_LENGTH, MAX_TRACE_INPUT_LENGTH, RIGHT_READ_NODE, ReverseObservation,
        TraceOperation, observe_insertion_sort, observe_is_sorted, observe_reverse,
        trace_is_sorted,
    };

    #[test]
    fn matches_native_result_and_counts_adjacent_comparisons() {
        let fixtures: &[(&[i32], IsSortedObservation)] = &[
            (
                &[],
                IsSortedObservation {
                    sorted: true,
                    comparisons: 0,
                    first_inversion: None,
                },
            ),
            (
                &[7],
                IsSortedObservation {
                    sorted: true,
                    comparisons: 0,
                    first_inversion: None,
                },
            ),
            (
                &[-2, 0, 0, 4],
                IsSortedObservation {
                    sorted: true,
                    comparisons: 3,
                    first_inversion: None,
                },
            ),
            (
                &[1, 2, 5, 4, 6],
                IsSortedObservation {
                    sorted: false,
                    comparisons: 3,
                    first_inversion: Some(3),
                },
            ),
        ];

        for (values, expected) in fixtures {
            assert_eq!(observe_is_sorted(values), Ok(*expected));
        }
    }

    #[test]
    fn rejects_inputs_beyond_the_browser_limit() {
        let values = vec![0; MAX_INPUT_LENGTH + 1];
        assert_eq!(observe_is_sorted(&values), Err(MAX_INPUT_LENGTH + 1));
        assert_eq!(observe_insertion_sort(&values), Err(MAX_INPUT_LENGTH + 1));
        assert_eq!(observe_reverse(&values), Err(MAX_INPUT_LENGTH + 1));
    }

    #[test]
    fn insertion_sort_matches_native_order_and_exposes_stability() {
        assert_eq!(
            observe_insertion_sort(&[2, 1, 2, 1]),
            Ok(InsertionSortObservation {
                values: vec![1, 1, 2, 2],
                original_indices: vec![1, 3, 0, 2],
                comparisons: 5,
                swaps: 3,
            })
        );
        assert_eq!(
            observe_insertion_sort(&[]),
            Ok(InsertionSortObservation {
                values: vec![],
                original_indices: vec![],
                comparisons: 0,
                swaps: 0,
            })
        );
        assert_eq!(
            observe_insertion_sort(&[1, 2, 3]),
            Ok(InsertionSortObservation {
                values: vec![1, 2, 3],
                original_indices: vec![0, 1, 2],
                comparisons: 2,
                swaps: 0,
            })
        );
    }

    #[test]
    fn reverse_matches_native_result_and_exact_structural_counts() {
        let fixtures: &[(&[i32], &[i32], u32)] = &[
            (&[], &[], 0),
            (&[7], &[7], 0),
            (&[1, 2], &[2, 1], 1),
            (&[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1], 2),
            (&[1, 2, 3, 4, 5, 6], &[6, 5, 4, 3, 2, 1], 3),
        ];

        for (input, expected, swaps) in fixtures {
            assert_eq!(
                observe_reverse(input),
                Ok(ReverseObservation {
                    values: expected.to_vec(),
                    reads: swaps * 2,
                    writes: swaps * 2,
                    swaps: *swaps,
                })
            );
        }
    }

    #[test]
    fn observed_reverse_is_an_involution() {
        let input = [4, -1, 7, 7, 2];
        let first = observe_reverse(&input).unwrap();
        let second = observe_reverse(&first.values).unwrap();

        assert_eq!(second.values, input);
    }

    #[test]
    fn is_sorted_trace_links_every_event_to_the_exact_ast_operation() {
        use atlas::ast::{SemanticOperation, is_sorted_ast};

        let trace = trace_is_sorted(&[1, 2, 5, 4, 6]).unwrap();
        assert!(!trace.sorted);
        assert_eq!(trace.first_inversion, Some(3));
        assert_eq!(trace.events.len(), 9);
        assert_eq!(
            trace
                .events
                .iter()
                .map(|event| event.node_id)
                .collect::<Vec<_>>(),
            [
                LEFT_READ_NODE,
                RIGHT_READ_NODE,
                COMPARE_NODE,
                LEFT_READ_NODE,
                RIGHT_READ_NODE,
                COMPARE_NODE,
                LEFT_READ_NODE,
                RIGHT_READ_NODE,
                COMPARE_NODE,
            ]
        );

        let ast = is_sorted_ast();
        for event in &trace.events {
            let expected = match event.operation {
                TraceOperation::Read => SemanticOperation::Read,
                TraceOperation::Compare => SemanticOperation::Compare,
            };
            assert_eq!(ast.operation_by_id(event.node_id), Some(expected));
        }
        let comparison = trace.events.last().unwrap();
        assert_eq!(comparison.left_index, 2);
        assert_eq!(comparison.right_index, Some(3));
        assert_eq!(comparison.ordering, Some(1));
    }

    #[test]
    fn is_sorted_trace_is_bounded_to_explore_inputs() {
        assert!(trace_is_sorted(&vec![0; MAX_TRACE_INPUT_LENGTH]).is_ok());
        assert_eq!(
            trace_is_sorted(&vec![0; MAX_TRACE_INPUT_LENGTH + 1]),
            Err(MAX_TRACE_INPUT_LENGTH + 1)
        );
    }

    #[test]
    fn is_sorted_trace_distinguishes_complete_scan_from_early_stop() {
        let complete = trace_is_sorted(&[7, 7, 7, 7]).unwrap();
        assert!(complete.sorted);
        assert_eq!(complete.events.len(), 9);

        let stopped = trace_is_sorted(&[5, -1, 5, 3]).unwrap();
        assert!(!stopped.sorted);
        assert_eq!(stopped.first_inversion, Some(1));
        assert_eq!(stopped.events.len(), 3);
    }
}
