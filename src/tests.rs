use std::collections::HashSet;

use super::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Person {
    name: String,
    age: u16,
}

impl AutoMapped for Person {
    type Key = String;
    type Value = u16;

    fn split(self) -> (Self::Key, Self::Value) {
        (self.name, self.age)
    }

    fn join((name, age): (Self::Key, Self::Value)) -> Self {
        Self { name, age }
    }
}

/// A struct just to ensure that the proper traits are defined
#[derive(Default, Debug, PartialEq, Eq)]
struct Outer(AutoHashMap<Person>);

#[test]
fn auto_hashmap() {
    let mut hashmap = AutoHashMap::<Person>::new();
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
    assert_eq!(hashmap.insert(bob2.clone()), Some(bob1.age));
    assert_eq!(
        hashmap
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .map(Person::join)
            .collect::<HashSet<Person>>(),
        vec![bob2.clone(), ruth.clone()].into_iter().collect()
    );
    assert_eq!(hashmap.len(), 2);
    assert_eq!(hashmap.get_cloned("Bob".to_string()), Some(bob2.clone()));
    assert_eq!(hashmap.remove("Bob".into()), Some(bob2.age));
    assert_eq!(hashmap.remove("Ruth".into()), Some(ruth.age));
}

#[test]
fn auto_btreemap() {
    let mut hashmap = AutoBTreeMap::<Person>::new();
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
    assert_eq!(hashmap.insert(bob2.clone()), Some(bob1.age));
    assert_eq!(
        hashmap.values().collect::<HashSet<&u16>>(),
        vec![&bob2.age, &ruth.age].into_iter().collect()
    );
    assert_eq!(hashmap.len(), 2);
    assert_eq!(hashmap.get("Bob"), Some(&bob2.age));
    assert_eq!(hashmap.remove("Bob"), Some(bob2.age));
    assert_eq!(hashmap.remove("Ruth"), Some(ruth.age));
}
