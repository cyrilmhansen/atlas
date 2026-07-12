#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
/// Copies elements into stable matching and rejected partitions.
pub fn partition_copy<T, F>(values: &[T], mut predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    let mut matching = Vec::new();
    let mut rejected = Vec::new();
    for value in values {
        if predicate(value) {
            matching.push(value.clone());
        } else {
            rejected.push(value.clone());
        }
    }
    (matching, rejected)
}

#[cfg(feature = "alloc")]
/// Replaces two caller-provided buffers with stable partitions.
///
/// Existing capacities are reused and may grow when insufficient.
pub fn partition_copy_into<T, F>(
    values: &[T],
    matching: &mut Vec<T>,
    rejected: &mut Vec<T>,
    mut predicate: F,
) where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    matching.clear();
    rejected.clear();
    for value in values {
        if predicate(value) {
            matching.push(value.clone());
        } else {
            rejected.push(value.clone());
        }
    }
}

/// Partitions a slice in place and returns the first rejected position.
///
/// Matching elements precede the returned boundary. Relative order is not
/// preserved.
pub fn partition_in_place<T, F>(values: &mut [T], mut predicate: F) -> usize
where
    F: FnMut(&T) -> bool,
{
    let (mut left, mut right) = (0, values.len());
    while left < right {
        while left < right && predicate(&values[left]) {
            left += 1;
        }
        while left < right && !predicate(&values[right - 1]) {
            right -= 1;
        }
        if left < right {
            values.swap(left, right - 1);
            left += 1;
            right -= 1;
        }
    }
    left
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use alloc::{vec, vec::Vec};

    use super::partition_in_place;
    #[cfg(feature = "alloc")]
    use super::{partition_copy, partition_copy_into};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Item {
        accepted: bool,
        position: usize,
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn copy_preserves_order_in_both_partitions() {
        let values = [
            Item {
                accepted: false,
                position: 0,
            },
            Item {
                accepted: true,
                position: 1,
            },
            Item {
                accepted: false,
                position: 2,
            },
            Item {
                accepted: true,
                position: 3,
            },
        ];

        let (matching, rejected) = partition_copy(&values, |item| item.accepted);

        assert_eq!(
            matching
                .iter()
                .map(|item| item.position)
                .collect::<Vec<_>>(),
            [1, 3]
        );
        assert_eq!(
            rejected
                .iter()
                .map(|item| item.position)
                .collect::<Vec<_>>(),
            [0, 2]
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn copy_handles_empty_all_and_no_matches() {
        let empty: [i32; 0] = [];
        assert_eq!(partition_copy(&empty, |_| true), (vec![], vec![]));
        assert_eq!(partition_copy(&[1, 2], |_| true), (vec![1, 2], vec![]));
        assert_eq!(partition_copy(&[1, 2], |_| false), (vec![], vec![1, 2]));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn copy_into_replaces_contents_and_reuses_capacities() {
        let values = [1, 2, 3, 4];
        let mut matching = Vec::with_capacity(values.len());
        let mut rejected = Vec::with_capacity(values.len());
        matching.push(99);
        rejected.push(98);
        let capacities = (matching.capacity(), rejected.capacity());

        partition_copy_into(&values, &mut matching, &mut rejected, |value| {
            value % 2 == 0
        });

        assert_eq!(matching, [2, 4]);
        assert_eq!(rejected, [1, 3]);
        assert_eq!((matching.capacity(), rejected.capacity()), capacities);
    }

    #[test]
    fn in_place_returns_a_valid_boundary_and_permutation() {
        let input = [1, 2, 3, 4, 5, 6];
        let mut values = input;

        let boundary = partition_in_place(&mut values, |value| value % 2 == 0);

        assert!(values[..boundary].iter().all(|value| value % 2 == 0));
        assert!(values[boundary..].iter().all(|value| value % 2 != 0));
        let mut actual = values;
        actual.sort();
        assert_eq!(actual, input);
    }

    #[test]
    fn in_place_does_not_guarantee_stability() {
        let mut values = [
            Item {
                accepted: false,
                position: 0,
            },
            Item {
                accepted: true,
                position: 1,
            },
            Item {
                accepted: true,
                position: 2,
            },
        ];

        let boundary = partition_in_place(&mut values, |item| item.accepted);

        assert_eq!(boundary, 2);
        assert_eq!(values[0].position, 2);
        assert_eq!(values[1].position, 1);
    }

    #[test]
    fn in_place_handles_empty_all_and_no_matches() {
        let mut empty: [i32; 0] = [];
        assert_eq!(partition_in_place(&mut empty, |_| true), 0);

        let mut all = [1, 2];
        assert_eq!(partition_in_place(&mut all, |_| true), 2);

        let mut none = [1, 2];
        assert_eq!(partition_in_place(&mut none, |_| false), 0);
    }
}
