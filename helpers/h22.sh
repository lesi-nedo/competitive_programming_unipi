#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source  "${SCRIPT_DIR}/rust_test.sh"

TEST_FOLDER="test_handson22"
RUST_EXEC="hands_on22"

run_test "$TEST_FOLDER" "$RUST_EXEC"
