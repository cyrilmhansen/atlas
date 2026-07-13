use core::cmp::Ordering;

use atlas_algorithms::is_sorted::is_sorted_by;
use wasm_bindgen::prelude::*;

pub const MAX_INPUT_LENGTH: usize = 4_096;

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

#[cfg(test)]
mod tests {
    use super::{IsSortedObservation, MAX_INPUT_LENGTH, observe_is_sorted};

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
    }
}
