use atlas_algorithms::deduplicate::deduplicate_quadratic;
use atlas_algorithms::filter::filter_in_place;
use atlas_algorithms::insertion_sort::insertion_sort_by;

/// Generated orchestration for `atlas compose cleanup --rust`.
pub fn cleanup<F>(values: &mut Vec<i32>, predicate: F) -> Vec<i32>
where
    F: FnMut(&i32) -> bool,
{
    // Step 1 mutates the caller's vector and does not allocate.
    filter_in_place(values, predicate);

    // Step 2 sorts that same vector and does not allocate.
    insertion_sort_by(values, i32::cmp);

    // Step 3 explicitly creates the required unique output vector.
    deduplicate_quadratic(values)
}

fn main() {
    let mut values = vec![4, -1, 2, 4, 2, 7, -3];
    let output = cleanup(&mut values, |value| *value >= 0);

    assert_eq!(values, [2, 2, 4, 4, 7]);
    assert_eq!(output, [2, 4, 7]);
    println!("{output:?}");
}
