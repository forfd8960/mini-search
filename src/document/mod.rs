use std::collections::HashMap;

use crate::errors::MSErrors;

pub struct Document {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
}

pub trait DocumentParser {
    fn parse(&self, input: &str) -> Result<Document, MSErrors>;
    fn extract_text(&self, document: &Document) -> String;
}
