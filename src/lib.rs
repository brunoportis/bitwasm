use std::collections::HashMap;

use console_error_panic_hook;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct BitmapIndex {
    index: HashMap<String, Vec<u32>>,
}

#[wasm_bindgen]
impl BitmapIndex {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BitmapIndex {
        BitmapIndex {
            index: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn insert(&mut self, key: String, id: u32) {
        let entry = self.index.entry(key).or_insert(Vec::new());

        if id as usize / 32 >= entry.len() {
            entry.resize(id as usize / 32 + 1, 0);
        }

        entry[id as usize / 32] |= 1 << (id % 32);
    }

    #[wasm_bindgen]
    pub fn get(&self, key: &str, id: usize) -> bool {
        if let Some(entry) = self.index.get(key) {
            let word_index = id / 32;
            if word_index < entry.len() {
                // aqui estamos criando uma máscara com um único bit setado para 1
                // ex. se id = 10, entao (1 << (10 % 32)) = (1 << 10) = 2^10 = 1024 -> 0000 0000 0000 0000 0000 0100 0000 0000
                let mask = 1 << (id % 32);

                // aqui estamos pegando o valor da word no índice word_index
                let value = entry[word_index];

                // aqui estamos usando o operador AND bit a bit para verificar se o bit está setado para 1
                let masked_value = value & mask;

                // true se o bit estiver setado para 1
                return masked_value != 0;
            }
        }
        false
    }

    #[wasm_bindgen]
    pub fn list(&self, key: &str) -> Vec<u32> {
        if let Some(entry) = self.index.get(key) {
            let mut result = Vec::new();
            for (i, word) in entry.iter().enumerate() {
                let mut mask = 1;
                for _ in 0..32 {
                    if word & mask != 0 {
                        result.push(i as u32 * 32 + mask.trailing_zeros());
                    }
                    mask <<= 1;
                }
            }
            return result;
        }
        Vec::new()
    }

    #[wasm_bindgen]
    pub fn list_keys(&self, key: &str) -> Vec<u32> {
        self.index.get(key).unwrap_or(&Vec::new()).clone()
    }

    #[wasm_bindgen]
    pub fn batch_insert(&mut self, key: String, ids: Vec<u32>) {
        for id in ids {
            self.insert(key.clone(), id);
        }
    }

    #[wasm_bindgen]
    pub fn get_as_binary(&self, key: &str) -> Vec<String> {
        let entry = self.index.get(key).unwrap();
        let mut result = Vec::new();
        for word in entry {
            result.push(format!("{:032b}", word));
        }
        result
    }

    #[wasm_bindgen]
    pub fn and_operation(&self, key1: &str, key2: &str) -> Vec<u32> {
        let entry1 = self.index.get(key1).unwrap();
        let entry2 = self.index.get(key2).unwrap();
        let mut result = Vec::new();
        for (i, word) in entry1.iter().enumerate() {
            // result.push(word & entry2[i]);
            for j in 0..32 {
                if (word & (1 << j)) != 0 && (entry2[i] & (1 << j)) != 0 {
                    result.push(i as u32 * 32 + j);
                }
            }
        }
        result
    }

    #[wasm_bindgen]
    pub fn or_operation(&self, key1: &str, key2: &str) -> Vec<u32> {
        let mut result = Vec::new();
        if let (Some(entry1), Some(entry2)) = (self.index.get(key1), self.index.get(key2)) {
            let len = entry1.len().max(entry2.len());
            for i in 0..len {
                let word1 = entry1.get(i).unwrap_or(&0);
                let word2 = entry2.get(i).unwrap_or(&0);
                for j in 0..32 {
                    if (word1 & (1 << j)) != 0 || (word2 & (1 << j)) != 0 {
                        result.push(i as u32 * 32 + j);
                    }
                }
            }
        }
        result
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut index = BitmapIndex::new();

    index.batch_insert("tem_pendencias".to_string(), [1, 3, 5, 7, 9].to_vec());
    index.batch_insert("is_admin".to_string(), [7, 256, 512, 1024].to_vec());

    log(&format!(
        "com pendencias: {:?}",
        index.list("tem_pendencias")
    ));
    log(&format!("admins: {:?}", index.list("is_admin")));

    log(&format!(
        "Pendencias: {:?}",
        index.list_keys("tem_pendencias")
    ));
    log(&format!("Admins: {:?}", index.list_keys("is_admin")));

    log(&format!("512 é admin?: {:?}", index.get("is_admin", 512)));

    log(&format!("Binários:"));
    log(&format!("{:?}", index.get_as_binary("tem_pendencias")));
    log(&format!("{:?}", index.get_as_binary("is_admin")));

    log(&format!(
        "Admins e com pendencias: {:?}",
        index.and_operation("tem_pendencias", "is_admin")
    ));

    log(&format!(
        "Admins ou com pendencias: {:?}",
        index.or_operation("is_admin", "tem_pendencias")
    ));
}
