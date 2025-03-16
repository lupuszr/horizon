use std::collections::HashMap;

use crate::errors::AppError;

pub enum TicketQuery {
    One(String),            // Query for one ticket by document_id
    Many(Vec<String>),      // Query for many tickets by a list of document_ids
    All,                    // Query for all tickets
    ByBucket(String),       // Query for read ticket by bucket name
    ByBuckets(Vec<String>), // Query for read tickets by multiple bucket names
    ByBucketPrefix(String), // Query for read tickets by bucket name prefix
}

#[derive(Debug)]
pub struct NamespaceLookupTable {
    document_to_bucket: HashMap<String, String>,
    bucket_to_document: HashMap<String, String>,
    document_read_ticket: HashMap<String, String>,
    document_write_ticket: HashMap<String, String>,
}

impl NamespaceLookupTable {
    pub fn new() -> Self {
        Self {
            document_to_bucket: HashMap::new(),
            bucket_to_document: HashMap::new(),
            document_read_ticket: HashMap::new(),
            document_write_ticket: HashMap::new(),
        }
    }

    pub fn insert(&mut self, document_id: String, bucket_name: String) {
        self.document_to_bucket
            .insert(document_id.clone(), bucket_name.clone());
        self.bucket_to_document.insert(bucket_name, document_id);
    }

    pub fn insert_read_ticket(&mut self, document_id: String, ticket: String) {
        self.document_read_ticket
            .insert(document_id.clone(), ticket.clone());
    }

    pub fn insert_read_ticket_by_bucket(
        &mut self,
        bucket: &str,
        ticket: String,
    ) -> Result<(), AppError> {
        let Some(document_id) = self.get_by_bucket_name(bucket) else {
            return Err(AppError::S3NoBucket);
        };

        self.document_read_ticket
            .insert(document_id.clone(), ticket.clone());

        Ok(())
    }

    pub fn insert_write_ticket(&mut self, document_id: String, ticket: String) {
        self.document_write_ticket
            .insert(document_id.clone(), ticket.clone());
    }

    pub fn insert_write_ticket_by_bucket(
        &mut self,
        bucket: &str,
        ticket: String,
    ) -> Result<(), AppError> {
        let Some(document_id) = self.get_by_bucket_name(bucket) else {
            return Err(AppError::S3NoBucket);
        };

        self.document_write_ticket
            .insert(document_id.clone(), ticket.clone());

        Ok(())
    }

    pub fn query_read_tickets(&self, query: TicketQuery) -> HashMap<String, String> {
        match query {
            TicketQuery::One(document_id) => {
                // Return a single ticket by document_id
                self.document_read_ticket
                    .iter()
                    .filter(|(key, _)| *key == &document_id)
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect()
            }
            TicketQuery::Many(document_ids) => {
                // Return tickets for multiple document IDs
                document_ids
                    .iter()
                    .filter_map(|id| {
                        self.document_read_ticket
                            .get(id)
                            .map(|ticket| (id.clone(), ticket.clone()))
                    })
                    .collect()
            }
            TicketQuery::All => {
                // Return all read tickets
                self.document_read_ticket.clone()
            }
            TicketQuery::ByBucket(bucket_name) => {
                // Query for read ticket by bucket name
                if let Some(document_id) = self.get_by_bucket_name(&bucket_name) {
                    self.document_read_ticket
                        .iter()
                        .filter(|(key, _)| *key == document_id)
                        .map(|(key, value)| (key.clone(), value.clone()))
                        .collect()
                } else {
                    HashMap::new()
                }
            }
            TicketQuery::ByBuckets(bucket_names) => {
                // Query for read tickets by multiple bucket names
                bucket_names
                    .iter()
                    .filter_map(|bucket_name| {
                        self.get_by_bucket_name(bucket_name).map(|document_id| {
                            self.document_read_ticket
                                .get(document_id)
                                .map(|ticket| (document_id.clone(), ticket.clone()))
                        })
                    })
                    .flatten()
                    .collect()
            }
            TicketQuery::ByBucketPrefix(prefix) => {
                // Query for read tickets by bucket name prefix
                self.bucket_to_document
                    .iter()
                    .filter(|(bucket_name, _)| bucket_name.starts_with(&prefix))
                    .filter_map(|(_bucket_name, document_id)| {
                        self.document_read_ticket
                            .get(document_id)
                            .map(|ticket| (document_id.clone(), ticket.clone()))
                    })
                    .collect()
            }
        }
    }

    pub fn get_by_document_id(&self, key: &str) -> Option<&String> {
        self.document_to_bucket.get(key)
    }

    pub fn get_by_bucket_name(&self, value: &str) -> Option<&String> {
        self.bucket_to_document.get(value)
    }

    pub fn remove_by_document_id(&mut self, key: &str) {
        if let Some(value) = self.document_to_bucket.remove(key) {
            self.bucket_to_document.remove(&value);
        }
    }

    pub fn remove_by_bucket_name(&mut self, value: &str) {
        if let Some(key) = self.bucket_to_document.remove(value) {
            self.document_to_bucket.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_one_ticket() {
        let mut namespace = NamespaceLookupTable::new();

        namespace.insert("doc1".to_string(), "bucket1".to_string());
        namespace.insert_read_ticket("doc1".to_string(), "ticket1".to_string());

        // Query for one ticket
        let result = namespace.query_read_tickets(TicketQuery::One("doc1".to_string()));
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("doc1"), Some(&"ticket1".to_string()));
    }

    #[test]
    fn test_query_many_tickets() {
        let mut namespace = NamespaceLookupTable::new();

        namespace.insert("doc1".to_string(), "bucket1".to_string());
        namespace.insert_read_ticket("doc1".to_string(), "ticket1".to_string());
        namespace.insert("doc2".to_string(), "bucket2".to_string());
        namespace.insert_read_ticket("doc2".to_string(), "ticket2".to_string());

        // Query for multiple tickets
        let result = namespace.query_read_tickets(TicketQuery::Many(vec![
            "doc1".to_string(),
            "doc2".to_string(),
        ]));
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("doc1"), Some(&"ticket1".to_string()));
        assert_eq!(result.get("doc2"), Some(&"ticket2".to_string()));
    }

    #[test]
    fn test_query_all_tickets() {
        let mut namespace = NamespaceLookupTable::new();

        namespace.insert("doc1".to_string(), "bucket1".to_string());
        namespace.insert_read_ticket("doc1".to_string(), "ticket1".to_string());
        namespace.insert("doc2".to_string(), "bucket2".to_string());
        namespace.insert_read_ticket("doc2".to_string(), "ticket2".to_string());

        // Query for all tickets
        let result = namespace.query_read_tickets(TicketQuery::All);
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("doc1"), Some(&"ticket1".to_string()));
        assert_eq!(result.get("doc2"), Some(&"ticket2".to_string()));
    }

    #[test]
    fn test_query_by_bucket() {
        let mut namespace = NamespaceLookupTable::new();

        namespace.insert("doc1".to_string(), "bucket1".to_string());
        namespace.insert_read_ticket("doc1".to_string(), "ticket1".to_string());
        namespace.insert("doc2".to_string(), "bucket2".to_string());
        namespace.insert_read_ticket("doc2".to_string(), "ticket2".to_string());

        // Query for ticket by bucket
        let result = namespace.query_read_tickets(TicketQuery::ByBucket("bucket1".to_string()));
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("doc1"), Some(&"ticket1".to_string()));
    }

    #[test]
    fn test_query_by_multiple_buckets() {
        let mut namespace = NamespaceLookupTable::new();

        namespace.insert("doc1".to_string(), "bucket1".to_string());
        namespace.insert_read_ticket("doc1".to_string(), "ticket1".to_string());
        namespace.insert("doc2".to_string(), "bucket2".to_string());
        namespace.insert_read_ticket("doc2".to_string(), "ticket2".to_string());
        namespace.insert("doc3".to_string(), "bucket3".to_string());
        namespace.insert_read_ticket("doc3".to_string(), "ticket3".to_string());

        // Query for tickets by multiple buckets
        let result = namespace.query_read_tickets(TicketQuery::ByBuckets(vec![
            "bucket1".to_string(),
            "bucket2".to_string(),
        ]));
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("doc1"), Some(&"ticket1".to_string()));
        assert_eq!(result.get("doc2"), Some(&"ticket2".to_string()));
    }

    #[test]
    fn test_query_by_bucket_prefix() {
        let mut namespace = NamespaceLookupTable::new();

        namespace.insert("doc1".to_string(), "bucket1".to_string());
        namespace.insert_read_ticket("doc1".to_string(), "ticket1".to_string());
        namespace.insert("doc2".to_string(), "bucket2".to_string());
        namespace.insert_read_ticket("doc2".to_string(), "ticket2".to_string());
        namespace.insert("doc3".to_string(), "bucket3".to_string());
        namespace.insert_read_ticket("doc3".to_string(), "ticket3".to_string());

        // Query for tickets by bucket prefix
        let result =
            namespace.query_read_tickets(TicketQuery::ByBucketPrefix("bucket".to_string()));
        assert_eq!(result.len(), 3);
        assert_eq!(result.get("doc1"), Some(&"ticket1".to_string()));
        assert_eq!(result.get("doc2"), Some(&"ticket2".to_string()));
        assert_eq!(result.get("doc3"), Some(&"ticket3".to_string()));
    }

    #[test]
    fn test_query_by_empty_buckets() {
        let namespace = NamespaceLookupTable::new();

        // Query for tickets by empty bucket list
        let result = namespace.query_read_tickets(TicketQuery::ByBuckets(vec![]));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_query_by_non_existing_bucket() {
        let mut namespace = NamespaceLookupTable::new();

        namespace.insert("doc1".to_string(), "bucket1".to_string());
        namespace.insert_read_ticket("doc1".to_string(), "ticket1".to_string());

        // Query for ticket by non-existing bucket
        let result =
            namespace.query_read_tickets(TicketQuery::ByBucket("non_existing_bucket".to_string()));
        assert_eq!(result.len(), 0);
    }
}
