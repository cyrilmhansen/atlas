use core::cmp::Ordering;
use core::fmt;

/// Error returned when caller-provided merge-sort storage is too small.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScratchTooSmall {
    pub required: usize,
    pub provided: usize,
}

impl fmt::Display for ScratchTooSmall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "merge-sort scratch has {} element(s); {} required",
            self.provided, self.required
        )
    }
}

/// Stably sorts a slice using one `O(n)` auxiliary allocation.
#[cfg(feature = "alloc")]
pub fn merge_sort_by<T, F>(values: &mut [T], mut compare: F)
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    let mut scratch = values.to_vec();
    merge_sort_by_with_scratch(values, &mut scratch, &mut compare)
        .expect("internally allocated scratch has the input length");
}

/// Stably sorts a slice using caller-provided auxiliary storage.
///
/// For inputs of length two or greater, `scratch` must contain at least as many
/// elements as `values`. A size error is returned before `values` is mutated.
pub fn merge_sort_by_with_scratch<T, F>(
    values: &mut [T],
    scratch: &mut [T],
    mut compare: F,
) -> Result<(), ScratchTooSmall>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    if values.len() < 2 {
        return Ok(());
    }
    if scratch.len() < values.len() {
        return Err(ScratchTooSmall {
            required: values.len(),
            provided: scratch.len(),
        });
    }

    sort_recursive(values, &mut scratch[..values.len()], &mut compare);
    Ok(())
}

fn sort_recursive<T, F>(values: &mut [T], scratch: &mut [T], compare: &mut F)
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    if values.len() < 2 {
        return;
    }

    let middle = values.len() / 2;
    {
        let (left, right) = values.split_at_mut(middle);
        let (left_scratch, right_scratch) = scratch.split_at_mut(middle);
        sort_recursive(left, left_scratch, compare);
        sort_recursive(right, right_scratch, compare);
    }

    if compare(&values[middle - 1], &values[middle]) != Ordering::Greater {
        return;
    }

    let (mut left, mut right) = (0, middle);
    for output in scratch.iter_mut() {
        let take_right = left == middle
            || (right < values.len() && compare(&values[right], &values[left]) == Ordering::Less);

        if take_right {
            *output = values[right].clone();
            right += 1;
        } else {
            *output = values[left].clone();
            left += 1;
        }
    }
    values.clone_from_slice(scratch);
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use alloc::{vec, vec::Vec};

    #[cfg(feature = "alloc")]
    use super::merge_sort_by;
    use super::{ScratchTooSmall, merge_sort_by_with_scratch};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Item {
        key: i32,
        original_position: usize,
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn sorts_and_preserves_the_input_permutation() {
        let input = vec![5, -1, 5, 3, 0, -8, 3];
        let mut actual = input.clone();

        merge_sort_by(&mut actual, i32::cmp);

        assert!(actual.windows(2).all(|pair| pair[0] <= pair[1]));
        let mut expected = input;
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn is_stable() {
        let mut values = vec![
            Item {
                key: 2,
                original_position: 0,
            },
            Item {
                key: 1,
                original_position: 1,
            },
            Item {
                key: 2,
                original_position: 2,
            },
            Item {
                key: 1,
                original_position: 3,
            },
        ];

        merge_sort_by(&mut values, |left, right| left.key.cmp(&right.key));

        let positions: Vec<_> = values
            .iter()
            .map(|item| (item.key, item.original_position))
            .collect();
        assert_eq!(positions, [(1, 1), (1, 3), (2, 0), (2, 2)]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn handles_empty_and_single_element_slices() {
        let mut empty: Vec<i32> = Vec::new();
        merge_sort_by(&mut empty, i32::cmp);

        let mut one = vec![42];
        merge_sort_by(&mut one, i32::cmp);

        assert!(empty.is_empty());
        assert_eq!(one, [42]);
    }

    #[test]
    fn caller_scratch_sorts_without_allocation() {
        let input = [5, -1, 5, 3, 0, -8, 3];
        let mut values = input;
        let mut scratch = [0; 7];

        merge_sort_by_with_scratch(&mut values, &mut scratch, i32::cmp).unwrap();

        assert!(values.windows(2).all(|pair| pair[0] <= pair[1]));
        let mut expected = input;
        expected.sort();
        assert_eq!(values, expected);
    }

    #[test]
    fn caller_scratch_preserves_stability() {
        let mut values = [
            Item {
                key: 2,
                original_position: 0,
            },
            Item {
                key: 1,
                original_position: 1,
            },
            Item {
                key: 2,
                original_position: 2,
            },
            Item {
                key: 1,
                original_position: 3,
            },
        ];
        let mut scratch = values.clone();

        merge_sort_by_with_scratch(&mut values, &mut scratch, |left, right| {
            left.key.cmp(&right.key)
        })
        .unwrap();

        assert_eq!(
            values[0],
            Item {
                key: 1,
                original_position: 1
            }
        );
        assert_eq!(
            values[1],
            Item {
                key: 1,
                original_position: 3
            }
        );
        assert_eq!(
            values[2],
            Item {
                key: 2,
                original_position: 0
            }
        );
        assert_eq!(
            values[3],
            Item {
                key: 2,
                original_position: 2
            }
        );
    }

    #[test]
    fn insufficient_scratch_does_not_mutate_input() {
        let original = [4, 3, 2, 1];
        let mut values = original;
        let mut scratch = [0; 3];

        let error = merge_sort_by_with_scratch(&mut values, &mut scratch, i32::cmp).unwrap_err();

        assert_eq!(
            error,
            ScratchTooSmall {
                required: 4,
                provided: 3,
            }
        );
        assert_eq!(values, original);
    }
}
