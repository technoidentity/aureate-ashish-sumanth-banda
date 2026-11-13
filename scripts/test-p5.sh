#!/usr/bin/env bash
set -euo pipefail

# Find the repository root relative to this script.
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

# Keep compiled files outside the repository.
build_dir="$(mktemp -d "${TMPDIR:-/tmp}/p5-tests.XXXXXX")"
trap 'rm -rf "$build_dir"' EXIT

echo "=== Immutable list tests ==="

CARGO_TARGET_DIR="$build_dir/cargo-target" \
    cargo test --locked \
    --manifest-path week5/week5/immutable_list/Cargo.toml

echo "=== Echo unit tests ==="

rustc --edition=2021 --test \
    week5/echo_standalone/echo_core.rs \
    -o "$build_dir/echo-unit-tests"

"$build_dir/echo-unit-tests"

echo "=== Echo integration tests ==="

rustc --edition=2021 \
    week5/echo_standalone/echo.rs \
    -o "$build_dir/echo"

rustc --edition=2021 --test \
    week5/echo_standalone/echo_cli_tests.rs \
    -o "$build_dir/echo-cli-tests"

ECHO_BIN="$build_dir/echo" "$build_dir/echo-cli-tests"

echo "=== Word-count unit tests ==="

rustc --edition=2021 --test \
    week5/word_count/word_count_standalone/core.rs \
    -o "$build_dir/word-count-unit-tests"

"$build_dir/word-count-unit-tests"

echo "=== Word-count integration tests ==="

rustc --edition=2021 \
    week5/word_count/word_count_standalone/main.rs \
    -o "$build_dir/word-count"

rustc --edition=2021 --test \
    week5/word_count/word_count_standalone/cli_tests.rs \
    -o "$build_dir/word-count-cli-tests"

# These tests find sample.txt relative to their working directory.
(
    cd "$repo_root/week5/word_count/word_count_standalone"

    WORD_COUNT_BIN="$build_dir/word-count" \
        "$build_dir/word-count-cli-tests"
)

echo "All configured P5 tests passed."
