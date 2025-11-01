use std::collections::HashMap;

pub struct SymbolTable {
    pub map: HashMap<String, i32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        map.insert("SP".to_string(), 0);
        map.insert("LCL".to_string(), 1);
        map.insert("ARG".to_string(), 2);
        map.insert("THIS".to_string(), 3);
        map.insert("THAT".to_string(), 4);
        
        for i in 0..16 {
            map.insert(format!("R{}", i), i);
        }
        
        map.insert("SCREEN".to_string(), 16384);
        map.insert("KBD".to_string(), 24576);

        SymbolTable { map }
    }

    pub fn add_entry(&mut self, symbol: &str, address: i32) {
        self.map.insert(symbol.to_string(), address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.map.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<i32> {
        self.map.get(symbol).copied()
    }
}
