use atlas_algorithms::deduplicate::deduplicate_quadratic;
use atlas_algorithms::insertion_sort::insertion_sort_by;

/// Generated orchestration for `atlas compose unique-sort --rust`.
pub fn unique_sort(values: &mut [i32]) -> Vec<i32> {
    // Step 1 mutates the caller's sequence and does not allocate.
    insertion_sort_by(values, i32::cmp);

    // Step 2 explicitly creates the required unique output vector.
    deduplicate_quadratic(values)
}

fn main() {
    let mut values = vec![4, 2, 4, 7, 2];
    let output = unique_sort(&mut values);

    assert_eq!(values, [2, 2, 4, 4, 7]);
    assert_eq!(output, [2, 4, 7]);
    println!("{output:?}");
}
