//! # tcm_library_core
//!
//! Pure Rust computation, indexing, and multi-dimensional clinical retrieval core
//! for Traditional Chinese Medicine (TCM) classical and modern texts.

pub mod error;
pub mod index;
pub mod matcher;
pub mod models;
pub mod parser;

// Top-level unified exports
pub use error::{CoreError, Result};
pub use index::{locate_manifest_path, CorpusIndex};
pub use matcher::search_corpus;
pub use models::{
    BookChapterEntryItem, BookChapterTreeItem, BookSummaryItem, CategoryTreeItem, CorpusConditions,
    CorpusImage, CorpusManifest, CorpusMetadata, CorpusStatus, ManifestCategory, ManifestEntry,
    ManifestSubcategory, SearchQuery, SearchResultItem, SubcategoryTreeItem, TcmEntryDetail,
};
pub use parser::parse_markdown_entry;
