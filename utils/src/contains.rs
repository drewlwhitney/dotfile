//! Utilities for checking whether lists contain or do not contain other lists.

/// Whether `container` contains all items in `contains`.
pub fn contains_all<T: PartialEq>(container: &[T], contains: &[T]) -> bool {
    contains.iter().all(|item| container.contains(&item))
}

/// Whether `container` has none of the items in `contains`.
pub fn contains_none<T: PartialEq>(container: &[T], contains: &[T]) -> bool {
    contains.iter().all(|item| !container.contains(&item))
}
