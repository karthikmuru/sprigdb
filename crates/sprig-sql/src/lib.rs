//! # sprig-sql
//!
//! Turns SQL text into an abstract syntax tree (AST). It has two parts: a
//! tokenizer that splits text into words and symbols, and a hand-written
//! recursive-descent parser that builds the tree. This crate knows nothing
//! about storage.
//!
//! The dialect follows SQLite wherever possible, plus sprigdb's own
//! `BRANCH`, `USE`, `DIFF`, `MERGE`, `DISCARD` and `DRY RUN`.
//!
//! Book chapter: "From text to AST".
