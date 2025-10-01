use std::collections::HashMap;

use super::tokenizer::{Language, Token, Tokenizer};

pub struct PostingsList {
    pub documents: Vec<DocumentPosting>,
    pub total_frequency: u64,
}

pub struct DocumentPosting {
    pub doc_id: u64,
    pub term_frequency: u32,
    pub positions: Vec<usize>, // for phrase queries
}

// Type alias for document ID
pub type DocId = usize;

// Represents a single occurrence of a term in a document
#[derive(Debug, PartialEq)]
pub struct Posting {
    pub doc_id: DocId,
    pub positions: Vec<usize>,        // Token positions in the document
    pub offsets: Vec<(usize, usize)>, // Character offsets in the original text
}

pub struct InvertedIndex {
    index: HashMap<String, Vec<Posting>>,
    tokenizer: Tokenizer,
}

impl InvertedIndex {
    // Create a new inverted index with a given tokenizer
    pub fn new(tokenizer: Tokenizer) -> Self {
        InvertedIndex {
            index: HashMap::new(),
            tokenizer,
        }
    }

    // Add a document to the index
    pub fn index_document(&mut self, doc_id: DocId, text: &str) {
        let tokens = self.tokenizer.tokenize(text);

        // Group tokens by term to build postings
        let mut term_positions: HashMap<String, (Vec<usize>, Vec<(usize, usize)>)> = HashMap::new();

        for token in tokens {
            let entry = term_positions
                .entry(token.term)
                .or_insert((Vec::new(), Vec::new()));
            entry.0.push(token.position);
            entry.1.push(token.offset);
        }

        // Update the inverted index
        for (term, (positions, offsets)) in term_positions {
            let posting = Posting {
                doc_id,
                positions,
                offsets,
            };
            self.index
                .entry(term)
                .or_insert_with(Vec::new)
                .push(posting);
        }
    }

    // Retrieve postings for a given term
    pub fn get_postings(&self, term: &str) -> Option<&Vec<Posting>> {
        self.index.get(term)
    }

    // Get all indexed terms (useful for debugging or query processing)
    pub fn terms(&self) -> impl Iterator<Item = &String> {
        self.index.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_document() {
        let tokenizer = Tokenizer::new(Language::English);
        let mut index = InvertedIndex::new(tokenizer);

        let doc_id = 1;
        let text = "The quick fox jumps";
        index.index_document(doc_id, text);

        // Check postings for "quick"
        let postings = index.get_postings("quick").unwrap();
        assert_eq!(postings.len(), 1);
        assert_eq!(
            postings[0],
            Posting {
                doc_id: 1,
                positions: vec![0],
                offsets: vec![(4, 9)],
            }
        );

        // Check postings for "fox"
        let postings = index.get_postings("fox").unwrap();
        assert_eq!(postings.len(), 1);
        assert_eq!(
            postings[0],
            Posting {
                doc_id: 1,
                positions: vec![1],
                offsets: vec![(10, 13)],
            }
        );

        // Check postings for "jump" (stemmed from "jumps")
        let postings = index.get_postings("jump").unwrap();
        assert_eq!(postings.len(), 1);
        assert_eq!(
            postings[0],
            Posting {
                doc_id: 1,
                positions: vec![2],
                offsets: vec![(14, 19)],
            }
        );

        // Check for stop word "the" (should not be indexed)
        assert_eq!(index.get_postings("the"), None);
    }

    #[test]
    fn test_multiple_documents() {
        let tokenizer = Tokenizer::new(Language::English);
        let mut index = InvertedIndex::new(tokenizer);

        index.index_document(1, "The quick fox");
        index.index_document(2, "Fox jumps high");

        // Check postings for "fox"
        let postings = index.get_postings("fox").unwrap();
        assert_eq!(postings.len(), 2);
        assert_eq!(
            postings[0],
            Posting {
                doc_id: 1,
                positions: vec![1],
                offsets: vec![(10, 13)],
            }
        );
        assert_eq!(
            postings[1],
            Posting {
                doc_id: 2,
                positions: vec![0],
                offsets: vec![(0, 3)],
            }
        );

        // Check postings for "jump" (stemmed from "jumps")
        let postings = index.get_postings("jump").unwrap();
        assert_eq!(postings.len(), 1);
        assert_eq!(
            postings[0],
            Posting {
                doc_id: 2,
                positions: vec![1],
                offsets: vec![(4, 9)],
            }
        );
    }

    #[test]
    fn test_empty_document() {
        let tokenizer = Tokenizer::new(Language::English);
        let mut index = InvertedIndex::new(tokenizer);

        index.index_document(1, "");
        assert_eq!(index.terms().count(), 0);
    }
}
