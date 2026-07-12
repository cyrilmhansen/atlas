#![no_std]

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

pub mod binary_search;
#[cfg(feature = "alloc")]
pub mod deduplicate;
#[cfg(feature = "alloc")]
pub mod filter;
pub mod insertion_sort;
pub mod is_sorted;
pub mod linear_search;
pub mod maximum;
pub mod merge_sort;
#[cfg(feature = "alloc")]
pub mod merge_sorted;
pub mod minimum;
pub mod partition;
pub mod reverse;
