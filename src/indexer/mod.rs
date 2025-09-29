use std::collections::HashMap;

use crate::document::Document;

pub struct InvertedIndex {
    // term -> list of documents containing it
    index: HashMap<String, PostingsList>,
    // document metadata
    documents: HashMap<u64, Document>,
}

pub struct PostingsList {
    pub documents: Vec<DocumentPosting>,
    pub total_frequency: u64,
}

pub struct DocumentPosting {
    pub doc_id: u64,
    pub term_frequency: u32,
    pub positions: Vec<usize>, // for phrase queries
}
