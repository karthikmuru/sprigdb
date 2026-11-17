//! # sprig-pager
//!
//! The lowest layer of sprigdb. The pager treats the database file as an
//! array of fixed-size pages and reads or writes one page at a time. It keeps
//! recently used pages in memory (the buffer cache) so it does not have to go
//! to disk for every read.
//!
//! This is the only crate in sprigdb that may use `unsafe`. Every `unsafe`
//! block must carry a `// SAFETY:` comment explaining why it is sound.
//!
//! Book chapter: "Pages and the pager" (book/src/explanation/pager.md).

#![allow(unsafe_code)] // Allowed only in this crate; see CONTRIBUTING.md.

/// The size of every page in the database file, in bytes.
///
/// ```
/// assert_eq!(sprig_pager::PAGE_SIZE, 4096);
/// ```
pub const PAGE_SIZE: usize = 4096;

/// The number of a page in the database file. Page 0 is the first page.
pub type PageId = u64;
