use alloc::vec::Vec;

/// Copies matching elements into a newly allocated vector while preserving order.
pub fn filter_copy<T, F>(values: &[T], mut predicate: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    values
        .iter()
        .filter(|value| predicate(*value))
        .cloned()
        .collect()
}

/// Replaces `output` with copies of matching elements while preserving order.
///
/// Existing capacity is reused and may grow when insufficient.
pub fn filter_copy_into<T, F>(values: &[T], output: &mut Vec<T>, mut predicate: F)
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    output.clear();
    output.extend(values.iter().filter(|value| predicate(*value)).cloned());
}

/// Removes non-matching elements in place while preserving order.
pub fn filter_in_place<T, F>(values: &mut Vec<T>, mut predicate: F)
where
    F: FnMut(&T) -> bool,
{
    values.retain(|value| predicate(value));
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use super::{filter_copy, filter_copy_into, filter_in_place};

    #[test]
    fn copy_preserves_matching_order() {
        let values = [4, 1, 2, 5, 2, 3];

        let result = filter_copy(&values, |value| value % 2 == 0);

        assert_eq!(result, [4, 2, 2]);
    }

    #[test]
    fn copy_handles_no_matches_and_empty_input() {
        assert!(filter_copy(&[1, 3, 5], |value| value % 2 == 0).is_empty());
        let empty: [i32; 0] = [];
        assert!(filter_copy(&empty, |_| true).is_empty());
    }

    #[test]
    fn copy_into_replaces_contents_and_reuses_capacity() {
        let values = [4, 1, 2, 5, 2, 3];
        let mut output = Vec::with_capacity(values.len());
        output.push(99);
        let capacity = output.capacity();

        filter_copy_into(&values, &mut output, |value| value % 2 == 0);

        assert_eq!(output, [4, 2, 2]);
        assert_eq!(output.capacity(), capacity);
    }

    #[test]
    fn copy_into_handles_all_and_no_matches() {
        let values = [1, 2, 3];
        let mut output = vec![99];
        filter_copy_into(&values, &mut output, |_| true);
        assert_eq!(output, values);

        filter_copy_into(&values, &mut output, |_| false);
        assert!(output.is_empty());
    }

    #[test]
    fn in_place_compaction_preserves_matching_order() {
        let mut values = vec![4, 1, 2, 5, 2, 3];

        filter_in_place(&mut values, |value| value % 2 == 0);

        assert_eq!(values, [4, 2, 2]);
    }

    #[test]
    fn in_place_compaction_handles_all_and_no_matches() {
        let mut values = vec![1, 2, 3];
        filter_in_place(&mut values, |_| true);
        assert_eq!(values, [1, 2, 3]);

        filter_in_place(&mut values, |_| false);
        assert!(values.is_empty());
    }
}
