use core::cmp::Ordering;

/// Returns the first maximum element, or `None` for an empty slice.
pub fn maximum_by<T, F>(values: &[T], mut compare: F) -> Option<&T>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut values = values.iter();
    let mut maximum = values.next()?;
    for candidate in values {
        if compare(candidate, maximum) == Ordering::Greater {
            maximum = candidate;
        }
    }
    Some(maximum)
}

#[cfg(test)]
mod tests {
    use super::maximum_by;

    #[derive(Debug, Eq, PartialEq)]
    struct Item {
        key: i32,
        original_position: usize,
    }

    #[test]
    fn returns_the_maximum_element() {
        let values = [7, -2, 4, 9, -1];

        let result = maximum_by(&values, i32::cmp);

        assert_eq!(result, Some(&9));
    }

    #[test]
    fn returns_the_first_equivalent_maximum() {
        let values = [
            Item {
                key: 2,
                original_position: 0,
            },
            Item {
                key: 3,
                original_position: 1,
            },
            Item {
                key: 3,
                original_position: 2,
            },
        ];

        let result = maximum_by(&values, |left, right| left.key.cmp(&right.key));

        assert_eq!(result.map(|item| item.original_position), Some(1));
    }

    #[test]
    fn handles_empty_and_single_element_slices() {
        let empty: [i32; 0] = [];
        assert_eq!(maximum_by(&empty, i32::cmp), None);

        let one = [42];
        assert_eq!(maximum_by(&one, i32::cmp), Some(&42));
    }
}
