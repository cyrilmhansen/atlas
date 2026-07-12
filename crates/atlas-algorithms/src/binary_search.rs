use core::cmp::Ordering;

/// Returns the first element equal to `needle` in a sorted slice.
///
/// `values` must be sorted according to `compare`.
pub fn binary_search_by<T, F>(values: &[T], needle: &T, mut compare: F) -> Option<usize>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let (mut left, mut right) = (0, values.len());
    while left < right {
        let middle = left + (right - left) / 2;
        if compare(&values[middle], needle) == Ordering::Less {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    (left < values.len() && compare(&values[left], needle) == Ordering::Equal).then_some(left)
}

#[cfg(test)]
mod tests {
    use super::binary_search_by;

    #[test]
    fn returns_the_first_matching_position() {
        let values = [1, 2, 2, 2, 7, 9];

        let result = binary_search_by(&values, &2, i32::cmp);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn returns_none_when_no_element_matches() {
        let values = [1, 2, 4, 7, 9];

        assert_eq!(binary_search_by(&values, &0, i32::cmp), None);
        assert_eq!(binary_search_by(&values, &3, i32::cmp), None);
        assert_eq!(binary_search_by(&values, &10, i32::cmp), None);
    }

    #[test]
    fn handles_empty_and_boundary_matches() {
        let empty: [i32; 0] = [];
        assert_eq!(binary_search_by(&empty, &1, i32::cmp), None);

        let values = [1, 3, 5];
        assert_eq!(binary_search_by(&values, &1, i32::cmp), Some(0));
        assert_eq!(binary_search_by(&values, &5, i32::cmp), Some(2));
    }
}
