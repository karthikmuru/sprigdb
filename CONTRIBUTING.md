# Contributing to sprigdb

sprigdb is meant to be read. A student should be able to read the whole
engine in a weekend. Every rule below protects that goal. When clarity and
speed conflict, clarity wins unless a benchmark says otherwise.

## Readable-Rust rules

1. **No lifetimes in public APIs.** Public functions take and return owned
   values or plain references that need no named lifetime. Clone if you
   must.
2. **No async in the core engine.** The pager, B+tree, SQL and executor
   crates are synchronous. Bindings may wrap them in async.
3. **Concrete types over clever generics.** Write `fn scan(&self) -> RowIter`,
   not `fn scan<I: Iterator<Item = R>, R: Row>(...)`. Use a generic only when
   there are at least two real callers that need it.
4. **`unsafe` only in `sprig-pager`.** The workspace denies `unsafe_code`.
   Inside the pager, every `unsafe` block needs a `// SAFETY:` comment.
5. **No custom macros.** Use functions. Derive macros from well-known crates
   (`serde`, `thiserror`) are fine.
6. **Few dependencies.** Each new dependency needs a sentence in the pull
   request saying why it is worth the reader's time.
7. **Name things for the reader.** `page_id`, not `pid`. `child_index`, not
   `ci`.
8. **Explain the why.** Comments say why the code is this way. The code
   already says what it does.

## Documentation is part of the product

- No layer is done until its book chapter is written.
- Every public item has a rustdoc comment. Add a runnable example where it
  helps; CI runs them, so they never go stale.
- Each crate has a short `README.md`: what it does, what to read first, what
  it depends on.
- A big design choice gets an ADR in `docs/adr/`.

## SQL compatibility

Follow SQLite's SQL wherever possible. Agents already write it well. New
commands should be guessable from their names.

## Before you open a pull request

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
mdbook build book
```

CI runs the same checks.

## License

By contributing, you agree that your work is dual-licensed under MIT or
Apache-2.0, like the rest of the project.
