//! # sprig-btree
//!
//! A copy-on-write B+tree stored in pager pages. "Copy-on-write" means a
//! page is never changed in place: a change writes a new copy of the page and
//! of every parent up to the root. The old root still points at the old
//! tree, which is why snapshots and branches are almost free.
//!
//! Book chapters: "The copy-on-write B+tree", "Snapshots for free" and
//! "Branching, diffing and merging".
