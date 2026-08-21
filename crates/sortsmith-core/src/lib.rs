#![forbid(unsafe_code)]

pub mod duplicates;
pub mod engine;
pub mod error;
pub mod journal;
pub mod models;
pub mod rules;
pub mod safety;

pub use duplicates::find_duplicates;
pub use engine::{execute_preview, preview_organization, undo_journal};
pub use error::{Result, SortSmithError};
pub use models::*;
