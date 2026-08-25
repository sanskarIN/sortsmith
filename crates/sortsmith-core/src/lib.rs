#![forbid(unsafe_code)]

pub mod duplicates;
pub mod engine;
pub mod error;
pub mod journal;
pub mod models;
pub mod rules;
pub mod safety;
pub mod scan_cache;

pub use duplicates::find_duplicates;
pub use engine::{execute_preview, preview_organization, undo_journal};
pub use error::{Result, SortSmithError};
pub use models::*;
pub use scan_cache::{preview_organization_cached, ScanCache, ScanCacheStats};
