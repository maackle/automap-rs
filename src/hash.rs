//! Simple helper to handle maps where the value contains the key

// NB: the traits can be simplified once associate type bounds stabilize
// https://github.com/rust-lang/rust/issues/52662

use std::collections::HashMap;

use crate::AutoMapped;

// Unused. Here for convenience if needed later.
trait AutoHashMapKey: Clone + std::hash::Hash + PartialEq + Eq {}
impl<T> AutoHashMapKey for T where T: Clone + std::hash::Hash + PartialEq + Eq {}

/// A HashMap whose values contain their keys
#[derive(shrinkwraprs::Shrinkwrap, derive_more::From, derive_more::Into)]
#[shrinkwrap(mutable, unsafe_ignore_visibility)]
pub struct AutoHashMap<T: AutoMapped>(HashMap<T::Key, T>)
where
    T::Key: Clone + std::hash::Hash + PartialEq + Eq;

impl<T: AutoMapped> AutoHashMap<T>
where
    T::Key: Clone + std::hash::Hash + PartialEq + Eq,
{
    /// Constructor
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Insert a key-value pair via just the value
    pub fn insert(&mut self, val: T) -> Option<T> {
        self.0.insert(val.key().clone(), val)
    }
}
