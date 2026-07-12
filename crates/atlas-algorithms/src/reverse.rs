/// Reverses a slice in place using symmetric swaps.
pub fn reverse_in_place<T>(values: &mut [T]) {
    for left in 0..values.len() / 2 {
        let right = values.len() - 1 - left;
        values.swap(left, right);
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_in_place;

    #[test]
    fn reverses_element_order() {
        let mut values = [1, 2, 3, 4, 5];

        reverse_in_place(&mut values);

        assert_eq!(values, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn handles_empty_and_single_element_slices() {
        let mut empty: [i32; 0] = [];
        reverse_in_place(&mut empty);

        let mut one = [42];
        reverse_in_place(&mut one);

        assert!(empty.is_empty());
        assert_eq!(one, [42]);
    }

    #[test]
    fn applying_twice_restores_the_input() {
        let input = [1, 2, 3, 4, 5, 6];
        let mut values = input;

        reverse_in_place(&mut values);
        reverse_in_place(&mut values);

        assert_eq!(values, input);
    }
}
