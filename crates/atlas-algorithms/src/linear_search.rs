/// Returns the position of the first element equivalent to `needle`.
pub fn linear_search_by<T, F>(values: &[T], needle: &T, mut equivalent: F) -> Option<usize>
where
    F: FnMut(&T, &T) -> bool,
{
    values
        .iter()
        .position(|candidate| equivalent(candidate, needle))
}

#[cfg(test)]
mod tests {
    use super::linear_search_by;

    #[test]
    fn returns_the_first_matching_position() {
        let values = [4, 2, 7, 2, 9];

        let result = linear_search_by(&values, &2, i32::eq);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn returns_none_when_no_element_matches() {
        let values = [4, 2, 7];

        let result = linear_search_by(&values, &5, i32::eq);

        assert_eq!(result, None);
    }

    #[test]
    fn handles_an_empty_slice() {
        let values: [i32; 0] = [];

        let result = linear_search_by(&values, &5, i32::eq);

        assert_eq!(result, None);
    }
}
