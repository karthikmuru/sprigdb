//! # sprig-exec
//!
//! Runs a parsed query. The catalog stores table schemas in a system
//! B+tree. The planner picks how to read the data (full scan or index scan).
//! The executor uses the Volcano iterator model: each operator (scan,
//! filter, project, limit) pulls rows one at a time from the one below it.
//!
//! Book chapters: "Planning" and "Executing a query".
