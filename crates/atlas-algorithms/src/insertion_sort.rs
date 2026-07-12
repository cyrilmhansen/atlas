use core::cmp::Ordering;

/// Stably sorts a slice in place without allocating.
pub fn insertion_sort_by<T, F>(values: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    for index in 1..values.len() {
        let mut current = index;
        while current > 0 && compare(&values[current], &values[current - 1]) == Ordering::Less {
            values.swap(current, current - 1);
            current -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort_by;

    #[derive(Debug, Eq, PartialEq)]
    struct Item {
        key: i32,
        original_position: usize,
    }

    #[test]
    fn sorts_and_preserves_the_input_permutation() {
        let input = [5, -1, 5, 3, 0, -8, 3];
        let mut actual = input;

        insertion_sort_by(&mut actual, i32::cmp);

        assert!(actual.windows(2).all(|pair| pair[0] <= pair[1]));
        let mut expected = input;
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn is_stable() {
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

        insertion_sort_by(&mut values, |left, right| left.key.cmp(&right.key));

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
    fn handles_empty_and_single_element_slices() {
        let mut empty: [i32; 0] = [];
        insertion_sort_by(&mut empty, i32::cmp);

        let mut one = [42];
        insertion_sort_by(&mut one, i32::cmp);

        assert!(empty.is_empty());
        assert_eq!(one, [42]);
    }
}
