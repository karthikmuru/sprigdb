# Introduction

sprigdb is a small SQL database that lives in one file. It is built for two
kinds of readers.

- **Agents and their builders.** Agents make mistakes. In sprigdb, an agent
  works in a branch. You look at the `DIFF`, then `MERGE` or `DISCARD`.
- **People learning how databases work.** The engine is small enough to read
  in a weekend. Each layer has a chapter here that explains the idea before
  the code.

These docs follow the [Diátaxis](https://diataxis.fr) framework:

| Section | Use it when you want to |
|---|---|
| Tutorials | learn by doing |
| How-to guides | get a task done |
| Reference | look something up |
| Explanation | understand how it works |

> sprigdb is in Phase 0. Most pages below are placeholders.
