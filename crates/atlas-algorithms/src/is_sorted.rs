use core::cmp::Ordering;

/// Returns whether adjacent elements are in nondecreasing order.
pub fn is_sorted_by<T, F>(values: &[T], mut compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    for pair in values.windows(2) {
        if compare(&pair[0], &pair[1]) == Ordering::Greater {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;

    use super::is_sorted_by;

    #[test]
    fn accepts_sorted_values_and_duplicates() {
        assert!(is_sorted_by(&[-2, 0, 0, 4, 9], i32::cmp));
    }

    #[test]
    fn rejects_an_adjacent_inversion() {
        assert!(!is_sorted_by(&[1, 2, 5, 4, 6], i32::cmp));
    }

    #[test]
    fn handles_empty_and_single_element_slices() {
        let empty: [i32; 0] = [];
        assert!(is_sorted_by(&empty, i32::cmp));
        assert!(is_sorted_by(&[42], i32::cmp));
    }

    #[test]
    fn stops_at_the_first_inversion() {
        let comparisons = Cell::new(0);

        let result = is_sorted_by(&[3, 2, 1, 0], |left, right| {
            comparisons.set(comparisons.get() + 1);
            left.cmp(right)
        });

        assert!(!result);
        assert_eq!(comparisons.get(), 1);
    }
}
