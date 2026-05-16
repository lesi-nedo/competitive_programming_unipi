#!/usr/bin/env bash

set -e

ROOT=".."
TEST_FOLDER="test_headson2"
TEST_PATH="${ROOT}/data/${TEST_FOLDER}"
RUST_EXEC="heads_on21"

if [ ! -e "$TEST_PATH" ]; then
  echo "Test folder doesn't exists. Checked: ${TEST_PATH}"
fi

cargo build --release --bin "${RUST_EXEC}"

for i in "$TEST_PATH"/input*.txt; do
  n=${i#"$TEST_PATH"/input}
  n=${n%.txt}

  diff -u \
    <("$ROOT"/target/release/"${RUST_EXEC}" < "$i") \
    "${TEST_PATH}/output${n}.txt"
done