//! A simple pattern to implement maps where the value type also contains the key type.
//! Implementations for `HashMap` and `BTreeMap` from `std::collections` are provided.
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

#![deny(missing_docs)]

#[cfg(test)]
mod tests;

use std::borrow::Borrow;

/// Trait that describes how to extract a key out of a value
pub trait AutoMapped {
    /// The key type
    type Key;

    /// The value type
    type Value;

    /// Split into key-value pair
    fn split(self) -> (Self::Key, Self::Value);

    /// Reconstitute from key and value parts
    fn join(pair: (Self::Key, Self::Value)) -> Self;
}

macro_rules! implementation {
    ($outer: ident, $inner: ident, $key_bounds: path, $value_bounds: path) => {
        use std::collections::$inner;

        /// A map whose values also contain their keys
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            shrinkwraprs::Shrinkwrap,
            derive_more::From,
            derive_more::Into,
        )]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[shrinkwrap(mutable, unsafe_ignore_visibility)]
        pub struct $outer<T: AutoMapped>($inner<T::Key, T::Value>)
        where
            T::Key: $key_bounds,
            T::Value: $value_bounds;

        impl<T: AutoMapped> Default for $outer<T>
        where
            T::Key: $key_bounds,
            T::Value: $value_bounds,
        {
            fn default() -> Self {
                Self($inner::default())
            }
        }

        impl<T: AutoMapped> $outer<T>
        where
            T::Key: $key_bounds + Clone,
            T::Value: $value_bounds,
        {
            /// Constructor
            pub fn new() -> Self {
                Self($inner::new())
            }

            /// Like `insert`, but returns a T, which requires cloning the key
            pub fn insert(&mut self, t: T) -> Option<T::Value>
            where
                T::Value: Clone,
            {
                let (k, v) = t.split();
                self.0.insert(k, v)
            }

            /// Like `insert`, but returns a T, which requires cloning the key
            pub fn insert_cloned(&mut self, t: T) -> Option<T>
            where
                T::Value: Clone,
            {
                let (k, v) = t.split();
                self.0
                    .insert(k.clone(), v)
                    .map(|val| T::join((k, val.to_owned())))
            }

            /// Like `remove`, but returns a T, which requires cloning the key
            pub fn remove_cloned<'a, C>(&mut self, k: C) -> Option<T>
            where
                T::Value: Clone,
                C: Clone + Borrow<T::Key> + $key_bounds,
                T::Key: Borrow<C>,
            {
                self.0
                    .remove(&k)
                    .map(|val| T::join((k.borrow().to_owned(), val.to_owned())))
            }

            /// Get an owned copy of the full type associated with this key.
            /// Requires cloning both key and value
            pub fn get_cloned<'a, C>(&self, k: C) -> Option<T>
            where
                T::Value: Clone,
                C: Clone + Borrow<T::Key> + $key_bounds,
                T::Key: Borrow<C>,
            {
                self.0
                    .get(&k)
                    .map(|val| T::join((k.borrow().to_owned(), val.to_owned())))
            }

            /// Pass-through for inner `into_iter`
            pub fn into_iter(self) -> impl Iterator<Item = (T::Key, T::Value)> {
                self.0.into_iter()
            }
        }
    };
}

// Implementations for both HashMap and BTreeMap are very similar
implementation!(AutoHashMap, HashMap, AutoHashMapKey, AutoHashMapValue);
implementation!(AutoBTreeMap, BTreeMap, AutoBTreeMapKey, AutoBTreeMapValue);

cfg_if::cfg_if! {
    if #[cfg(feature = "serde")] {
        /// The constraints on an AutoHashMap key
        pub trait AutoHashMapKey: serde::Serialize + serde::de::DeserializeOwned + std::hash::Hash + PartialEq + Eq {}
        impl<T> AutoHashMapKey for T where T: serde::Serialize + serde::de::DeserializeOwned + std::hash::Hash + PartialEq + Eq {}

        /// The constraints on an AutoBTreeMap key
        pub trait AutoBTreeMapKey: serde::Serialize + serde::de::DeserializeOwned + PartialOrd + Ord {}
        impl<T> AutoBTreeMapKey for T where T: serde::Serialize + serde::de::DeserializeOwned + PartialOrd + Ord {}

        /// The constraints on an AutoHashMap value
        pub trait AutoHashMapValue: serde::Serialize + serde::de::DeserializeOwned {}
        impl<T> AutoHashMapValue for T where T: serde::Serialize + serde::de::DeserializeOwned {}

        /// The constraints on an AutoBTreeMap value
        pub trait AutoBTreeMapValue: serde::Serialize + serde::de::DeserializeOwned {}
        impl<T> AutoBTreeMapValue for T where T: serde::Serialize + serde::de::DeserializeOwned {}
    } else {
        /// The constraints on an AutoHashMap key
        pub trait AutoHashMapKey: std::hash::Hash + PartialEq + Eq {}
        impl<T> AutoHashMapKey for T where T: std::hash::Hash + PartialEq + Eq {}

        /// The constraints on an AutoBTreeMap key
        pub trait AutoBTreeMapKey: PartialOrd + Ord {}
        impl<T> AutoBTreeMapKey for T where T: PartialOrd + Ord {}

        /// The constraints on an AutoHashMap Value
        pub trait AutoHashMapValue {}
        impl<T> AutoHashMapValue for T where T {}

        /// The constraints on an AutoBTreeMap Value
        pub trait AutoBTreeMapValue {}
        impl<T> AutoBTreeMapValue for T where T {}

    }
}
