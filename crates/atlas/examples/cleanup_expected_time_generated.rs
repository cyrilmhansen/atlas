use atlas_algorithms::deduplicate::deduplicate_hash;
use atlas_algorithms::filter::filter_copy;
use atlas_algorithms::merge_sort::merge_sort_by;

/// Generated orchestration for `atlas compose cleanup --goal expected-time --rust`.
pub fn cleanup_expected_time<F>(values: &[i32], predicate: F) -> Vec<i32>
where
    F: FnMut(&i32) -> bool,
{
    // Step 1 explicitly allocates and copies the filtered sequence.
    let mut filtered = filter_copy(values, predicate);

    // Step 2 mutates that copy and allocates merge-sort scratch storage.
    merge_sort_by(&mut filtered, i32::cmp);

    // Step 3 explicitly allocates the output and its internal hash set.
    deduplicate_hash(&filtered)
}

fn main() {
    let values = [4, -1, 2, 4, 2, 7, -3];
    let output = cleanup_expected_time(&values, |value| *value >= 0);

    assert_eq!(values, [4, -1, 2, 4, 2, 7, -3]);
    assert_eq!(output, [2, 4, 7]);
    println!("{output:?}");
}
