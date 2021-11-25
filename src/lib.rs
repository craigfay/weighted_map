use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::rc::Rc;

/// Any type that is used as a key for
/// WeightedMap must implement this type:
/// ```impl Key for String {}```
pub trait Key: Eq + Hash + Clone {}
impl Key for String {}

struct WeightedMap<K: Key, V> {
    key_to_value: HashMap<Rc<K>, V>,
    key_to_weight: HashMap<Rc<K>, u32>,
}

impl<K: Key, V> WeightedMap<K, V> {
    pub fn new() -> Self {
        Self {
            key_to_value: HashMap::new(),
            key_to_weight: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let rc_key = Rc::new(key);
        self.key_to_value.insert(Rc::clone(&rc_key), value);
        self.key_to_weight.insert(Rc::clone(&rc_key), 0);
    }

    pub fn add_weight(&mut self, key: K, weight: u32) {
        let rc_key = Rc::new(key);

        match self.key_to_weight.get_mut(&rc_key) {
            Some(old_weight) => *old_weight = old_weight.saturating_add(weight),
            None => (),
        };
    }

    pub fn get_weight(&mut self, key: K) -> Option<&u32> {
        let rc_key = Rc::new(key);
        self.key_to_weight.get(&rc_key)
    }
}

#[test]
fn basic_test() {
    let mut wm = WeightedMap::new();
    wm.insert("a".to_string(), "b".to_string());

    let w = wm.get_weight("a".to_string());
    assert_eq!(w, Some(&0));

}