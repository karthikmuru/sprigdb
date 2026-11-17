//! # sprigdb
//!
//! The public API. This is the crate users depend on, and the one the
//! Python, Node and WebAssembly bindings wrap. Everything else in the
//! workspace is an internal layer.
//!
//! Planned usage (not built yet):
//!
//! ```text
//! let db = sprigdb::open("notes.db")?;
//! db.execute("CREATE TABLE notes (id INTEGER PRIMARY KEY, body TEXT)")?;
//! let branch = db.branch("agent_1")?;
//! branch.execute("DELETE FROM notes WHERE id > 10")?;
//! println!("{}", branch.diff()?);
//! branch.merge()?;
//! ```
