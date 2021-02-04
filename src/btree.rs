//! Simple helper to handle maps where the value contains the key

// NB: the traits can be simplified once associate type bounds stabilize
// https://github.com/rust-lang/rust/issues/52662

use std::collections::BTreeMap;

use crate::AutoMapped;

// Unused. Here for convenience if needed later.
trait AutoBTreeMapKey: Clone + PartialOrd + Ord {}
impl<T> AutoBTreeMapKey for T where T: Clone + PartialOrd + Ord {}

/// A BTreeMap whose values contain their keys
#[derive(shrinkwraprs::Shrinkwrap, derive_more::From, derive_more::Into)]
#[shrinkwrap(mutable, unsafe_ignore_visibility)]
pub struct AutoBTreeMap<T: AutoMapped>(BTreeMap<T::Key, T>)
where
    T::Key: Clone + PartialOrd + Ord;

impl<T: AutoMapped> AutoBTreeMap<T>
where
    T::Key: Clone + PartialOrd + Ord,
{
    /// Constructor
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Insert a key-value pair via just the value
    pub fn insert(&mut self, val: T) -> Option<T> {
        self.0.insert(val.key().clone(), val)
    }
}
