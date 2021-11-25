use std::collections::{HashSet, HashMap};
use std::rc::Rc;

struct WeightedMap {
    // keys: HashSet<String>,
    key_to_value: HashMap<Rc<String>, ()>,
    key_to_weight: HashMap<Rc<String>, ()>,
}

impl WeightedMap {
    pub fn new() -> Self {
        Self {
            // keys: HashSet::new(),
            key_to_value: HashMap::new(),
            key_to_weight: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String) {
        let rc_key = Rc::new(key);
        self.key_to_value.insert(Rc::clone(&rc_key), ());
        self.key_to_weight.insert(Rc::clone(&rc_key), ());
    }
}

#[test]
fn basic_test() {
    let mut wm = WeightedMap::new();
    wm.insert("a".to_string());
}