use atlas_algorithms::binary_search::binary_search_by;
use atlas_algorithms::insertion_sort::insertion_sort_by;

/// Generated orchestration for `atlas compose find --rust`.
pub fn find(values: &mut [i32], needle: &i32) -> Option<usize> {
    // Step 1 visibly establishes the precondition required by binary search.
    insertion_sort_by(values, i32::cmp);

    // Step 2 reads the now-sorted vector without allocating.
    binary_search_by(values, needle, i32::cmp)
}

fn main() {
    let mut values = vec![4, 1, 2, 4];
    let index = find(&mut values, &4);

    assert_eq!(values, [1, 2, 4, 4]);
    assert_eq!(index, Some(2));
    println!("{index:?}");
}
