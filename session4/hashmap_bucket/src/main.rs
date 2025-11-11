use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct HashMapBucket<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> HashMapBucket<K, V>
where
    K: Eq + Hash,
{
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_default();
        values.push(value);
    }
}

fn main() {
    let mut bucket = HashMapBucket::new();
    bucket.insert("hello", 1);
    bucket.insert("hello", 2);
    bucket.insert("goodbye", 3);
    println!("{bucket:?}");
}
