use thiserror::Error;

#[derive(Debug, Error)]
pub enum MSErrors {
    #[error("Document not found")]
    DocumentNotFound,
    #[error("Indexing error: {0}")]
    IndexingError(String),
    #[error("Search error: {0}")]
    SearchError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}
