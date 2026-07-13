use atlas_algorithms::insertion_sort::insertion_sort_by;
use atlas_algorithms::partition::partition_copy;

pub fn partition_then_sort_matching<F>(values: &[i32], predicate: F) -> (Vec<i32>, Vec<i32>)
where
    F: FnMut(&i32) -> bool,
{
    let (mut matching, rejected) = partition_copy(values, predicate);
    insertion_sort_by(&mut matching, i32::cmp);
    (matching, rejected)
}

fn main() {
    let (matching, rejected) = partition_then_sort_matching(&[4, 1, 2, 5, 3], |v| v % 2 == 0);
    assert_eq!(matching, [2, 4]);
    assert_eq!(rejected, [1, 5, 3]);
}
