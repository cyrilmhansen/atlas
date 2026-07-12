use alloc::vec::Vec;
use core::cmp::Ordering;

/// Stably merges two sorted slices into a newly allocated vector.
pub fn merge_sorted<T, F>(left: &[T], right: &[T], compare: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    let capacity = merged_len(left, right);
    let mut output = Vec::with_capacity(capacity);
    merge_sorted_into(left, right, &mut output, compare);
    output
}

/// Replaces `output` with the stable merge of two sorted slices.
///
/// Existing capacity is reused and may grow when insufficient.
pub fn merge_sorted_into<T, F>(left: &[T], right: &[T], output: &mut Vec<T>, mut compare: F)
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    let required = merged_len(left, right);
    output.clear();
    output.reserve(required);

    let (mut left_index, mut right_index) = (0, 0);
    while left_index < left.len() && right_index < right.len() {
        if compare(&right[right_index], &left[left_index]) == Ordering::Less {
            output.push(right[right_index].clone());
            right_index += 1;
        } else {
            output.push(left[left_index].clone());
            left_index += 1;
        }
    }
    output.extend_from_slice(&left[left_index..]);
    output.extend_from_slice(&right[right_index..]);
}

fn merged_len<T>(left: &[T], right: &[T]) -> usize {
    left.len()
        .checked_add(right.len())
        .expect("merged sequence length exceeds usize")
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::{merge_sorted, merge_sorted_into};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Item {
        key: i32,
        source: char,
        position: usize,
    }

    #[test]
    fn merges_in_sorted_order_and_preserves_elements() {
        let left = [1, 3, 5, 8];
        let right = [2, 3, 7, 9];

        let result = merge_sorted(&left, &right, i32::cmp);

        assert_eq!(result, [1, 2, 3, 3, 5, 7, 8, 9]);
    }

    #[test]
    fn is_stable_across_and_within_inputs() {
        let left = [
            Item {
                key: 1,
                source: 'L',
                position: 0,
            },
            Item {
                key: 2,
                source: 'L',
                position: 1,
            },
            Item {
                key: 2,
                source: 'L',
                position: 2,
            },
        ];
        let right = [
            Item {
                key: 2,
                source: 'R',
                position: 0,
            },
            Item {
                key: 3,
                source: 'R',
                position: 1,
            },
        ];

        let result = merge_sorted(&left, &right, |a, b| a.key.cmp(&b.key));
        let origins: Vec<_> = result
            .iter()
            .map(|item| (item.source, item.position))
            .collect();

        assert_eq!(origins, [('L', 0), ('L', 1), ('L', 2), ('R', 0), ('R', 1)]);
    }

    #[test]
    fn handles_empty_inputs() {
        let empty: [i32; 0] = [];
        assert_eq!(merge_sorted(&empty, &empty, i32::cmp), []);
        assert_eq!(merge_sorted(&[1, 2], &empty, i32::cmp), [1, 2]);
        assert_eq!(merge_sorted(&empty, &[1, 2], i32::cmp), [1, 2]);
    }

    #[test]
    fn into_replaces_contents_and_reuses_capacity() {
        let left = [1, 3];
        let right = [2, 4];
        let mut output = Vec::with_capacity(left.len() + right.len());
        output.push(99);
        let capacity = output.capacity();

        merge_sorted_into(&left, &right, &mut output, i32::cmp);

        assert_eq!(output, [1, 2, 3, 4]);
        assert_eq!(output.capacity(), capacity);
    }
}
