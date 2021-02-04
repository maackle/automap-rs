//! Simple helper to handle maps where the value contains the key

use std::collections::{BTreeMap, HashMap};

/// A HashMap whose values contain their keys
#[derive(shrinkwraprs::Shrinkwrap, derive_more::From, derive_more::Into)]
#[shrinkwrap(mutable, unsafe_ignore_visibility)]
pub struct AutoHashMap<T: AutoHashMapped>(HashMap<T::Key, T>);

impl<T: AutoHashMapped> AutoHashMap<T> {
    /// Constructor
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

/// A BTreeMap whose values contain their keys
#[derive(shrinkwraprs::Shrinkwrap, derive_more::From, derive_more::Into)]
#[shrinkwrap(mutable, unsafe_ignore_visibility)]
pub struct AutoBTreeMap<T: AutoBTreeMapped>(BTreeMap<T::Key, T>);

impl<T: AutoBTreeMapped> AutoBTreeMap<T> {
    /// Constructor
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

/// Trait that describes how to get a Hashable key out of a value
pub trait AutoMapped {
    /// The key type
    type Key: Clone;

    /// Accessor for the key
    fn key(&self) -> &Self::Key;
}

/// Trait that describes how to get a Hashable key out of a value
pub trait AutoHashMapped {
    /// The key type
    type Key: Clone + std::hash::Hash + PartialEq + Eq;

    /// Accessor for the key
    fn key(&self) -> &Self::Key;
}

impl<T: AutoHashMapped> AutoHashMap<T> {
    /// Insert a key-value pair via just the value
    pub fn insert(&mut self, val: T) -> Option<T> {
        self.0.insert(val.key().clone(), val)
    }
}

/// Trait that describes how to get an Ordered key out of a value
pub trait AutoBTreeMapped {
    /// The key type
    type Key: Clone + PartialOrd + Ord;

    /// Accessor for the key
    fn key(&self) -> &Self::Key;
}

impl<T: AutoBTreeMapped> AutoBTreeMap<T> {
    /// Insert a key-value pair via just the value
    pub fn insert(&mut self, val: T) -> Option<T> {
        self.0.insert(val.key().clone(), val)
    }
}

impl<T: AutoMapped> AutoHashMapped for T
where
    T::Key: std::hash::Hash + PartialEq + Eq,
{
    type Key = T::Key;

    fn key(&self) -> &T::Key {
        self.key()
    }
}

impl<T: AutoMapped> AutoBTreeMapped for T
where
    T::Key: PartialOrd + Ord,
{
    type Key = T::Key;

    fn key(&self) -> &T::Key {
        self.key()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    struct Person {
        name: String,
        age: u16,
    }

    impl AutoMapped for Person {
        type Key = String;

        fn key(&self) -> &String {
            &self.name
        }
    }

    #[test]
    fn auto_hashmap() {
        let mut hashmap = AutoHashMap::new();
        let bob1 = Person {
            name: "Bob".into(),
            age: 23,
        };
        let bob2 = Person {
            name: "Bob".into(),
            age: 41,
        };
        let ruth = Person {
            name: "Ruth".into(),
            age: 32,
        };
        assert_eq!(hashmap.insert(bob1.clone()), None);
        assert_eq!(hashmap.insert(ruth.clone()), None);
        assert_eq!(hashmap.insert(bob2.clone()), Some(bob1.clone()));
        assert_eq!(
            hashmap.values().collect::<HashSet<&Person>>(),
            vec![&bob2, &ruth].into_iter().collect()
        );
        assert_eq!(hashmap.len(), 2);
        assert_eq!(hashmap.get("Bob"), Some(&bob2));
        assert_eq!(hashmap.remove("Bob"), Some(bob2));
        assert_eq!(hashmap.remove("Ruth"), Some(ruth));
    }
}
