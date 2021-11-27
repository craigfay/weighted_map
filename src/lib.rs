use std::collections::{HashMap, BTreeMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

/// Any type that is used as a key for
/// WeightedMap must implement this type:
/// ```impl Key for String {}```
pub trait Key: Eq + Hash + Clone {}
impl Key for String {}

pub struct WeightedMap<K: Key, V> {
    key_to_value: HashMap<Rc<K>, V>,
    key_to_weight: HashMap<Rc<K>, u32>,
    weight_to_key: BTreeMap<u32, HashSet<Rc<K>>>,
}

impl<K: Key, V> WeightedMap<K, V> {
    pub fn new() -> Self {
        Self {
            key_to_value: HashMap::new(),
            key_to_weight: HashMap::new(),
            weight_to_key: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let rc_key = Rc::new(key);
        self.key_to_value.insert(Rc::clone(&rc_key), value);
        self.key_to_weight.insert(Rc::clone(&rc_key), 0);

        let keys_with_same_weight = self.weight_to_key
            .entry(0)
            .or_insert(HashSet::new());

        keys_with_same_weight.insert(Rc::clone(&rc_key));
    }

    pub fn get_value(&mut self, key: K) -> Option<&V> {
        self.key_to_value.get(&Rc::new(key))
    }

    pub fn add_weight(&mut self, key: K, weight: u32) {
        match self.key_to_weight.get_mut(&Rc::new(key)) {
            Some(w) => *w = w.saturating_add(weight),
            None => (),
        };
    }

    pub fn subtract_weight(&mut self, key: K, weight: u32) {
        match self.key_to_weight.get_mut(&Rc::new(key)) {
            Some(w) => *w = w.saturating_sub(weight),
            None => (),
        };
    }

    pub fn get_weight(&mut self, key: K) -> Option<&u32> {
        self.key_to_weight.get(&Rc::new(key))
    }

    pub fn keys_with_weight(&self, weight: u32) -> Option<&HashSet<Rc<K>>> {
        self.weight_to_key.get(&weight)
    }
}

#[test]
fn basic_test() {
    let mut wm = WeightedMap::new();
    wm.insert("a".to_string(), "b".to_string());

    let w = wm.get_weight("a".to_string());
    assert_eq!(w, Some(&0));

    let v = wm.get_value("a".to_string());
    assert_eq!(v, Some(&"b".to_string()));

    wm.add_weight("a".to_string(), 3);
    let w = wm.get_weight("a".to_string());
    assert_eq!(w, Some(&3));

    wm.subtract_weight("a".to_string(), 1);
    let w = wm.get_weight("a".to_string());
    assert_eq!(w, Some(&2));

    let keys = wm.keys_with_weight(0).unwrap();
    assert_eq!(keys.len(), 1);
}