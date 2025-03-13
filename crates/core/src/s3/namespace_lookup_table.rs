use std::collections::HashMap;

#[derive(Debug)]
pub struct NamespaceLookupTable {
    forward: HashMap<String, String>,
    reverse: HashMap<String, String>,
}

impl NamespaceLookupTable {
    pub fn new() -> Self {
        Self {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    pub fn insert(&mut self, document_id: String, bucket_name: String) {
        self.forward
            .insert(document_id.clone(), bucket_name.clone());
        self.reverse.insert(bucket_name, document_id);
    }

    pub fn get_by_document_id(&self, key: &str) -> Option<&String> {
        self.forward.get(key)
    }

    pub fn get_by_bucket_name(&self, value: &str) -> Option<&String> {
        self.reverse.get(value)
    }

    pub fn remove_by_document_id(&mut self, key: &str) {
        if let Some(value) = self.forward.remove(key) {
            self.reverse.remove(&value);
        }
    }

    pub fn remove_by_bucket_name(&mut self, value: &str) {
        if let Some(key) = self.reverse.remove(value) {
            self.forward.remove(&key);
        }
    }
}
