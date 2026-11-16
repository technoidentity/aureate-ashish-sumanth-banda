# Running the practice examples
Run these commands from the repository root.
Requirements: Rust (`rustc` and `cargo`), Bash, and SQLite 3.
## Standalone Rust examples
Compile and run the generic comparison, iterator, filter, fold and closure
examples. Executables are written to a temporary directory.
```bash
example_build_dir=$(mktemp -d)
for source in \
  week4/chapter10/chap10_5.rs \
  week5/itr.rs \
  week5/flt.rs \
  week5/fld.rs \
  week5/sec.rs
do
  example_name=$(basename "$source" .rs)
  rustc --edition=2021 "$source" \
    -o "$example_build_dir/$example_name" || break
  "$example_build_dir/$example_name" || break
done
```
## SQL practice

Use SQLite to run the schema, seed data and query in that order.
This uses a fresh in-memory database, so it does not modify existing databases.

```bash
sqlite3 :memory: <<'SQL'
.bail on
PRAGMA foreign_keys = ON;
.read sql/schema.sql
.read sql/seed.sql
.read sql/practice.sql
SQL
```

The query should return `Ashish`.

## Pre-commit hook

Activate the tracked hook once in each clone:

```bash
git config --local core.hooksPath .githooks
```

The hook runs `scripts/test-p5.sh` before each commit and stops the commit
if the script fails. Run the same checks manually with:

```bash
bash scripts/test-p5.sh
```