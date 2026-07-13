#!/usr/bin/env bash


SCRITP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)";

source  "${SCRITP_DIR}/rust_test.sh"

TEST_FOLDER="test_handson32"
RUST_EXEC="hands_on32"

run_test "${TEST_FOLDER}" "${RUST_EXEC}"