use std::collections::HashSet;
use stemmer::Stemmer;

// Define supported languages (extendable for future use)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
}

// Token struct to hold term, position, and offset
#[derive(Debug, PartialEq)]
pub struct Token {
    pub term: String,
    pub position: usize,
    pub offset: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    language: Language,
    stop_words: HashSet<String>,
}

/*
Iterates through characters to track offsets precisely, handling UTF-8 correctly.
Converts letters to lowercase and collects them into words.
Splits on whitespace or punctuation, creating tokens when a word ends.
Applies stemming and filters stop words to produce clean tokens.
Tracks position (0-based index in the token sequence) and offset (start/end byte positions in the original text).
*/
impl Tokenizer {
    // Create a new Tokenizer with language and stop words
    pub fn new(language: Language) -> Self {
        let stop_words = match language {
            Language::English => {
                let words = vec![
                    "a", "an", "and", "are", "as", "at", "be", "by", "for", "from", "has", "he",
                    "in", "is", "it", "its", "of", "on", "that", "the", "to", "was", "were",
                    "will", "with",
                ];
                words
                    .into_iter()
                    .map(String::from)
                    .collect::<HashSet<String>>()
            }
        };
        Tokenizer {
            language,
            stop_words,
        }
    }

    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut position = 0;
        let mut char_iter = text.char_indices();
        let mut current_word = String::new();
        let mut start_offset = 0;

        // Initialize stemmer for English
        let mut stemmer = Stemmer::new("english").expect("Failed to initialize stemmer");

        while let Some((idx, ch)) = char_iter.next() {
            if ch.is_alphabetic() {
                current_word.push(ch.to_ascii_lowercase());
            } else if ch.is_whitespace() || ch.is_ascii_punctuation() {
                if !current_word.is_empty() {
                    // Process the current word
                    let stemmed = stemmer.stem(&current_word);
                    if !self.stop_words.contains(&stemmed) && !stemmed.is_empty() {
                        tokens.push(Token {
                            term: stemmed,
                            position,
                            offset: (start_offset, idx),
                        });
                        position += 1;
                    }
                    current_word.clear();
                }
                // Update start offset for the next word
                start_offset = idx + ch.len_utf8();
            }
        }

        // Handle the last word if it exists
        if !current_word.is_empty() {
            let stemmed = stemmer.stem(&current_word);
            if !self.stop_words.contains(&stemmed) && !stemmed.is_empty() {
                tokens.push(Token {
                    term: stemmed,
                    position,
                    offset: (start_offset, text.len()),
                });
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let tokenizer = Tokenizer::new(Language::English);
        let text = "The quick foxes jump!";
        let tokens = tokenizer.tokenize(text);
        let expected = vec![
            Token {
                term: String::from("quick"),
                position: 0,
                offset: (4, 9),
            },
            Token {
                term: String::from("fox"),
                position: 1,
                offset: (10, 15),
            },
            Token {
                term: String::from("jump"),
                position: 2,
                offset: (16, 20),
            },
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_empty() {
        let tokenizer = Tokenizer::new(Language::English);
        let tokens = tokenizer.tokenize("");
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_tokenize_stop_words() {
        let tokenizer = Tokenizer::new(Language::English);
        let text = "The and is";
        let tokens = tokenizer.tokenize(text);
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_tokenize_punctuation() {
        let tokenizer = Tokenizer::new(Language::English);
        let text = "Hello, world!!!";
        let tokens = tokenizer.tokenize(text);
        let expected = vec![
            Token {
                term: String::from("hello"),
                position: 0,
                offset: (0, 5),
            },
            Token {
                term: String::from("world"),
                position: 1,
                offset: (7, 12),
            },
        ];
        assert_eq!(tokens, expected);
    }
}
