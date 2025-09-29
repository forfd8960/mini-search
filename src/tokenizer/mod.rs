use std::{collections::HashSet, vec};

use crate::errors::MSErrors;

pub enum Language {
    English,
}

pub struct Tokenizer {
    language: Language,
    stop_words: HashSet<String>,
}

impl Tokenizer {
    pub fn tokenize(&self, text: &str) -> Result<Vec<Token>, MSErrors> {
        // Split text, normalize case, remove punctuation
        // Filter stop words, apply stemming
        Ok(Vec::new()) // Placeholder
    }
}

pub struct Token {
    pub term: String,
    pub position: usize,
    pub offset: (usize, usize),
}
