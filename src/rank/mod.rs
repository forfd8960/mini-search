use super::indexer::{DocId, InvertedIndex, Posting};
use super::tokenizer::{Language, Tokenizer};
use std::collections::{HashMap, HashSet};

// BM25Ranker struct to hold corpus statistics and parameters
pub struct BM25Ranker {
    tokenizer: Tokenizer,
    index: InvertedIndex,
    doc_lengths: HashMap<DocId, usize>, // Number of tokens per document
    avg_doc_length: f64,                // Average document length
    total_docs: usize,                  // Total number of documents
    k1: f64,                            // BM25 parameter for term frequency saturation
    b: f64,                             // BM25 parameter for length normalization
}

impl BM25Ranker {
    // Create a new BM25Ranker
    pub fn new(tokenizer: Tokenizer, index: InvertedIndex) -> Self {
        BM25Ranker {
            tokenizer,
            index,
            doc_lengths: HashMap::new(),
            avg_doc_length: 0.0,
            total_docs: 0,
            k1: 1.5, // Common default
            b: 0.75, // Common default
        }
    }

    // Add a document and update corpus statistics
    pub fn index_document(&mut self, doc_id: DocId, text: &str) {
        // Tokenize to get document length
        let tokens = self.tokenizer.tokenize(text);
        let doc_length = tokens.len();

        // Update document lengths and corpus stats
        self.doc_lengths.insert(doc_id, doc_length);
        self.total_docs += 1;
        self.update_avg_doc_length();

        // Delegate indexing to the inverted index
        self.index.index_document(doc_id, text);
    }

    // Update average document length
    fn update_avg_doc_length(&mut self) {
        if self.total_docs > 0 {
            let total_length: usize = self.doc_lengths.values().sum();
            self.avg_doc_length = total_length as f64 / self.total_docs as f64;
        }
    }

    // Compute IDF for a term
    fn compute_idf(&self, term: &str) -> f64 {
        let n_qi = self
            .index
            .get_postings(term)
            .map_or(0, |postings| postings.len());
        ((self.total_docs as f64 - n_qi as f64 + 0.5) / (n_qi as f64 + 0.5) + 1.0).ln()
    }

    // Compute BM25 score for a document given query terms
    fn compute_score(&self, doc_id: DocId, query_terms: &[String]) -> f64 {
        let doc_length = *self.doc_lengths.get(&doc_id).unwrap_or(&0) as f64;
        if doc_length == 0.0 {
            return 0.0;
        }

        let mut score = 0.0;
        for term in query_terms {
            if let Some(postings) = self.index.get_postings(term) {
                if let Some(posting) = postings.iter().find(|p| p.doc_id == doc_id) {
                    let tf = posting.positions.len() as f64; // Term frequency
                    let idf = self.compute_idf(term);
                    let numerator = tf * (self.k1 + 1.0);
                    let denominator =
                        tf + self.k1 * (1.0 - self.b + self.b * doc_length / self.avg_doc_length);
                    score += idf * numerator / denominator;
                }
            }
        }
        score
    }

    // Rank documents for a query
    pub fn rank(&self, query: &str) -> Vec<(DocId, f64)> {
        // Tokenize query and remove duplicates
        let query_tokens = self.tokenizer.tokenize(query);
        let query_terms: Vec<String> = query_tokens.into_iter().map(|t| t.term).collect();
        let unique_terms: HashSet<String> = HashSet::from_iter(query_terms.iter().cloned());

        // Collect documents containing any query term
        let mut candidate_docs: HashSet<DocId> = HashSet::new();
        for term in &unique_terms {
            if let Some(postings) = self.index.get_postings(term) {
                for posting in postings {
                    candidate_docs.insert(posting.doc_id);
                }
            }
        }

        // Compute scores for each candidate document
        let mut results: Vec<(DocId, f64)> = candidate_docs
            .into_iter()
            .map(|doc_id| {
                (
                    doc_id,
                    self.compute_score(
                        doc_id,
                        &unique_terms.clone().into_iter().collect::<Vec<_>>(),
                    ),
                )
            })
            .filter(|&(_, score)| score > 0.0)
            .collect();

        // Sort by score in descending order
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bm25_ranking() {
        let tokenizer = Tokenizer::new(Language::English);
        let index = InvertedIndex::new(tokenizer.clone());
        let mut ranker = BM25Ranker::new(tokenizer, index);

        // Index some documents
        ranker.index_document(1, "The quick fox jumps");
        ranker.index_document(2, "Fox jumps high");
        ranker.index_document(3, "Slow turtle walks");

        // Query for "fox jumps"
        let results = ranker.rank("fox jumps");

        // Expected: Doc 2 and Doc 1 should rank higher than Doc 3
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 2); // "Fox jumps high" should rank higher (more query terms, shorter doc)
        assert_eq!(results[1].0, 1);
        assert!(results[0].1 > results[1].1); // Doc 2 should have a higher score
    }

    #[test]
    fn test_empty_query() {
        let tokenizer = Tokenizer::new(Language::English);
        let index = InvertedIndex::new(tokenizer.clone());
        let ranker = BM25Ranker::new(tokenizer, index);

        let results = ranker.rank("");
        assert_eq!(results, vec![]);
    }

    #[test]
    fn test_no_relevant_docs() {
        let tokenizer = Tokenizer::new(Language::English);
        let index = InvertedIndex::new(tokenizer.clone());
        let mut ranker = BM25Ranker::new(tokenizer, index);

        ranker.index_document(1, "The quick fox");
        let results = ranker.rank("turtle");
        assert_eq!(results, vec![]);
    }
}
