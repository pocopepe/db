use std::collections::BTreeMap;

enum Value {
    Active(String),
    Tombstone,
}

pub struct MemTable {
    map: BTreeMap<String, Value>,
    live_bytes: usize,
    mem_bytes: usize,
}

impl MemTable {
    pub fn new() -> Self {
        Self { map: BTreeMap::new(), live_bytes: 0, mem_bytes: 0 }
    }

    pub fn put(&mut self, key: String, value: String) {
        if let Some(old) = self.map.get(&key) {
            match old {
                Value::Active(v) => {
                    self.live_bytes -= key.len() + v.len();
                    self.mem_bytes -= key.len() + v.len();
                }
                Value::Tombstone => self.mem_bytes -= key.len(),
            }
        }

        self.live_bytes += key.len() + value.len();
        self.mem_bytes += key.len() + value.len();
        self.map.insert(key, Value::Active(value));
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        match self.map.get(key) {
            Some(Value::Active(v)) => Some(v),
            _ => None,
        }
    }

    pub fn delete(&mut self, key: String) {
        match self.map.insert(key.clone(), Value::Tombstone) {
            Some(Value::Active(v)) => {
                self.live_bytes -= key.len() + v.len();
                self.mem_bytes -= v.len();
            }
            None => self.mem_bytes += key.len(),
            _ => {}
        }
    }

    pub fn live_bytes(&self) -> usize { self.live_bytes }
    pub fn mem_bytes(&self) -> usize { self.mem_bytes }
}