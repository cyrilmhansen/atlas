use atlas_algorithms::insertion_sort::insertion_sort_by;
use atlas_algorithms::merge_sorted::merge_sorted as merge_sorted_values;

/// Generated orchestration for `atlas compose merge-sorted --rust`.
pub fn merge_after_sort(left: &mut [i32], right: &mut [i32]) -> Vec<i32> {
    // Steps 1 and 2 establish the sorted-input preconditions in place.
    insertion_sort_by(left, i32::cmp);
    insertion_sort_by(right, i32::cmp);

    // Step 3 allocates the required stable merged output.
    merge_sorted_values(left, right, i32::cmp)
}

fn main() {
    let mut left = [5, 1, 3];
    let mut right = [4, 2, 3];
    let output = merge_after_sort(&mut left, &mut right);

    assert_eq!(left, [1, 3, 5]);
    assert_eq!(right, [2, 3, 4]);
    assert_eq!(output, [1, 2, 3, 3, 4, 5]);
    println!("{output:?}");
}
