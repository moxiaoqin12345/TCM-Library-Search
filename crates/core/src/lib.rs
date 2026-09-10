//! # tcm_library_core
//!
//! Pure Rust computation, indexing, and multi-dimensional clinical retrieval core
//! for Traditional Chinese Medicine (TCM) classical and modern texts.

pub mod compatibility;
pub mod diff;
pub mod error;
pub mod index;
pub mod matcher;
pub mod meridian;
pub mod models;
pub mod parser;

// Top-level unified exports
pub use compatibility::{
    check_herb_compatibility, CompatibilityAlert, IncompatibilitySeverity, IncompatibilityType,
};
pub use diff::{diff_texts, DiffChunk, DiffOp, TextDiffResult};
pub use error::{CoreError, Result};
pub use index::{locate_manifest_path, CorpusIndex};
pub use matcher::search_corpus;
pub use meridian::{
    find_acupoint, get_meridian_knowledge_base, recommend_acupoints_for_symptom, AcupointInfo,
    MeridianCategory, MeridianInfo, SpecificAcupointType,
};
pub use models::{
    BookChapterEntryItem, BookChapterTreeItem, BookSummaryItem, CategoryTreeItem, CorpusConditions,
    CorpusImage, CorpusManifest, CorpusMetadata, CorpusStatus, ManifestCategory, ManifestEntry,
    ManifestSubcategory, SearchQuery, SearchResultItem, SubcategoryTreeItem, TcmEntryDetail,
};
pub use parser::parse_markdown_entry;
