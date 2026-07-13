#!/usr/bin/env bash


SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source  "${SCRIPT_DIR}/rust_test.sh"

TEST_FOLDER="test_handson31"
RUST_EXEC="hands_on31"

run_test "${TEST_FOLDER}" "${RUST_EXEC}"