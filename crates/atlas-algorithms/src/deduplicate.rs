use alloc::vec::Vec;

#[cfg(feature = "hash-dedup")]
use core::hash::Hash;
#[cfg(feature = "hash-dedup")]
use hashbrown::HashSet;

/// Copies the first occurrence of each value using a quadratic scan.
pub fn deduplicate_quadratic<T>(values: &[T]) -> Vec<T>
where
    T: Clone + Eq,
{
    let mut output = Vec::new();
    for value in values {
        if !output.contains(value) {
            output.push(value.clone());
        }
    }
    output
}

/// Copies the first occurrence of each value using a hash set.
#[cfg(feature = "hash-dedup")]
pub fn deduplicate_hash<T>(values: &[T]) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    let mut output = Vec::with_capacity(values.len());
    deduplicate_hash_into(values, &mut output);
    output
}

/// Replaces `output` with first occurrences found using a hash set.
///
/// Existing output capacity is reused and may grow when insufficient. The
/// internal hash set always requires allocation for non-empty inputs.
#[cfg(feature = "hash-dedup")]
pub fn deduplicate_hash_into<T>(values: &[T], output: &mut Vec<T>)
where
    T: Clone + Eq + Hash,
{
    output.clear();
    let mut seen = HashSet::with_capacity(values.len());
    for value in values {
        if seen.insert(value) {
            output.push(value.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::hash::{Hash, Hasher};

    use super::deduplicate_quadratic;
    #[cfg(feature = "hash-dedup")]
    use super::{deduplicate_hash, deduplicate_hash_into};

    #[derive(Clone, Debug)]
    struct Item {
        key: i32,
        first_position: usize,
    }

    impl PartialEq for Item {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    impl Eq for Item {}

    impl Hash for Item {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.key.hash(state);
        }
    }

    fn repeated_items() -> [Item; 5] {
        [
            Item {
                key: 2,
                first_position: 0,
            },
            Item {
                key: 1,
                first_position: 1,
            },
            Item {
                key: 2,
                first_position: 2,
            },
            Item {
                key: 3,
                first_position: 3,
            },
            Item {
                key: 1,
                first_position: 4,
            },
        ]
    }

    #[test]
    fn quadratic_preserves_first_occurrences() {
        let result = deduplicate_quadratic(&repeated_items());
        let positions: Vec<_> = result.iter().map(|item| item.first_position).collect();

        assert_eq!(positions, [0, 1, 3]);
    }

    #[test]
    fn quadratic_handles_empty_and_unique_inputs() {
        let empty: [i32; 0] = [];
        assert!(deduplicate_quadratic(&empty).is_empty());
        assert_eq!(deduplicate_quadratic(&[1, 2, 3]), [1, 2, 3]);
    }

    #[test]
    #[cfg(feature = "hash-dedup")]
    fn hash_preserves_first_occurrences() {
        let result = deduplicate_hash(&repeated_items());
        let positions: Vec<_> = result.iter().map(|item| item.first_position).collect();

        assert_eq!(positions, [0, 1, 3]);
    }

    #[test]
    #[cfg(feature = "hash-dedup")]
    fn hash_handles_empty_and_duplicate_only_inputs() {
        let empty: [i32; 0] = [];
        assert!(deduplicate_hash(&empty).is_empty());
        assert_eq!(deduplicate_hash(&[2, 2, 2]), [2]);
    }

    #[test]
    #[cfg(feature = "hash-dedup")]
    fn hash_into_replaces_contents_and_reuses_output_capacity() {
        let values = [2, 1, 2, 3, 1];
        let mut output = Vec::with_capacity(values.len());
        output.push(99);
        let capacity = output.capacity();

        deduplicate_hash_into(&values, &mut output);

        assert_eq!(output, [2, 1, 3]);
        assert_eq!(output.capacity(), capacity);
    }
}
