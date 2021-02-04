//! Simple pattern to implement maps where the value type also contains the key type
//!
//! ```
//! use std::collections::HashMap;
//! use automap::{AutoHashMap, AutoMapped};
//!
//! // Let's say we want a `Person` to be keyed by their `name` in a HashMap
//! #[derive(Debug, Clone, Hash, PartialEq, Eq)]
//! struct Person {
//!     name: String,
//!     age: u16,
//! }
//!
//! // We can specify how to derive the key from the value
//! // As long as the Key type meets the bounds for a normal HashMap key, we
//! // can use this value in an AutoHashMap.
//! // (Similarly for BTreeMap.)
//! impl AutoMapped for Person {
//!     type Key = String;
//!
//!     fn key(&self) -> &Self::Key {
//!         &self.name
//!     }
//! }
//!
//! // Then, we can simply use an `AutoHashMap` to insert values directly.
//! let mut map = AutoHashMap::new();
//! let michelle = Person { name: "Michelle".into(), age: 37 };
//! map.insert(michelle.clone());
//!
//! // You can access all other normal HashMap methods directly:
//! assert_eq!(map.get("Michelle".into()), Some(&michelle));
//! assert_eq!(map.remove("Michelle".into()), Some(michelle));
//!
//! // We can also go From and Into a normal HashMap easily.
//! let inner: HashMap<_, _> = map.into();
//! let map: AutoHashMap<_> = inner.into();
//! ```

mod btree;
mod hash;

#[cfg(test)]
mod tests;

pub use btree::*;
pub use hash::*;

/// Trait that describes how to get a Hashable key out of a value
pub trait AutoMapped {
    /// The key type
    type Key;

    /// Accessor for the key
    fn key(&self) -> &Self::Key;
}
