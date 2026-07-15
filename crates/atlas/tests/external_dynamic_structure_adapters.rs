use std::collections::BinaryHeap;
use std::hash::{BuildHasherDefault, Hasher};

use dary_heap::QuaternaryHeap;
use hashbrown::{HashMap, HashSet};
use petgraph::unionfind::UnionFind;

#[derive(Default)]
struct ConstantHasher;

impl Hasher for ConstantHasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, _bytes: &[u8]) {}
}

type CollidingMap<K, V> = HashMap<K, V, BuildHasherDefault<ConstantHasher>>;

#[test]
fn petgraph_union_find_preserves_partition_across_interleaved_operations() {
    let mut partition = UnionFind::<u32>::new(6);

    assert!(partition.union(0, 1));
    assert!(partition.union(1, 2));
    assert!(!partition.union(0, 2));
    assert!(partition.union(3, 4));

    assert!(partition.equiv(0, 2));
    assert!(!partition.equiv(2, 3));

    let representative = partition.find_mut(2);
    assert_eq!(representative, partition.find(0));
    assert!(partition.equiv(0, 2));
    assert!(!partition.equiv(2, 3));
}

#[test]
fn binary_heap_operations_preserve_max_heap_behavior_and_capacity() {
    let mut heap = BinaryHeap::with_capacity(4);
    let reserved_capacity = heap.capacity();

    for value in [2, 7, 3, 5] {
        heap.push(value);
    }

    assert_eq!(heap.capacity(), reserved_capacity);
    assert_eq!(heap.peek(), Some(&7));
    assert_eq!(heap.pop(), Some(7));
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), None);
}

#[test]
fn dary_heap_push_preserves_max_order_duplicates_and_reserved_capacity() {
    let mut heap = QuaternaryHeap::with_capacity(5);
    let reserved_capacity = heap.capacity();

    for value in [2, 7, 3, 7] {
        heap.push(value);
        assert_eq!(heap.capacity(), reserved_capacity);
    }

    assert_eq!(heap.peek(), Some(&7));
    assert_eq!(heap.pop(), Some(7));
    heap.push(5);
    assert_eq!(heap.capacity(), reserved_capacity);
    assert_eq!(heap.pop(), Some(7));
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), None);
}

#[test]
fn dary_heap_push_grows_exhausted_storage() {
    let mut heap = QuaternaryHeap::new();
    assert_eq!(heap.capacity(), 0);

    heap.push(11);

    assert!(heap.capacity() > 0);
    assert_eq!(heap.pop(), Some(11));
}

#[test]
fn hashbrown_map_resolves_collisions_for_insert_get_and_remove() {
    let mut map = CollidingMap::with_capacity_and_hasher(4, BuildHasherDefault::default());
    let reserved_capacity = map.capacity();

    assert_eq!(map.insert("alpha", 1), None);
    assert_eq!(map.insert("beta", 2), None);
    assert_eq!(map.insert("gamma", 3), None);
    assert_eq!(map.capacity(), reserved_capacity);
    assert_eq!(map.get("beta"), Some(&2));
    assert_eq!(map.insert("beta", 20), Some(2));
    assert_eq!(map.remove("alpha"), Some(1));
    assert_eq!(map.get("beta"), Some(&20));
    assert_eq!(map.get("gamma"), Some(&3));
}

fn hashbrown_stable_deduplicate(values: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::with_capacity(values.len());
    values
        .iter()
        .copied()
        .filter(|value| seen.insert(*value))
        .collect()
}

#[test]
fn hashbrown_adapter_stably_deduplicates_without_atlas_algorithm_code() {
    assert_eq!(
        hashbrown_stable_deduplicate(&[3, 1, 3, 2, 1, 4]),
        vec![3, 1, 2, 4]
    );
    assert!(hashbrown_stable_deduplicate(&[]).is_empty());
}
