#!/usr/bin/env bash

ROOT=".."
TEST_FOLDER="test_headson22"
TEST_PATH="${ROOT}/data/${TEST_FOLDER}"
RUST_EXEC="heads_on22"


if [ ! -e "${TEST_PATH}" ]; then
  echo "THIS: ${TEST_PATH} does not exists. Fix it."
fi

cargo build --release --bin "${RUST_EXEC}"

for i in "${TEST_PATH}"/input*.txt; do
  n=${i#"${TEST_PATH}"/input}
  n=${n%.txt}

  diff -u \
    <("$ROOT"/target/release/"${RUST_EXEC}" < "$i") \
    "${TEST_PATH}/output${n}.txt"
done