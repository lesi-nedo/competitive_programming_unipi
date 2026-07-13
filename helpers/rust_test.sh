#!/usr/bin/env bash

 run_test() {
   local TEST_FOLDER="$1"
   local RUST_EXEC="$2"

   local SCRIPT_DIR
   SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

   local ROOT="${3:-${SCRIPT_DIR}/..}"
   local TEST_PATH="${ROOT}/data/${TEST_FOLDER}"

   local GREEN='\033[0;32m'
   local RED='\033[0;31m'
   local NC='\033[0m' # No Color

   if [ ! -e "${TEST_PATH}" ]; then
     echo "THIS: ${TEST_PATH} does not exist. Fix it."
     return 1
   fi

   cargo build --release --bin "${RUST_EXEC}" || return 1

   local pass_count=0
   local fail_count=0

   for i in "${TEST_PATH}"/input*.txt; do
     n=${i#"${TEST_PATH}"/input}
     n=${n%.txt}

     local diff_output
     diff_output=$(diff -u \
       <("$ROOT"/target/release/"${RUST_EXEC}" < "$i") \
       "${TEST_PATH}/output${n}.txt")

     if [ -z "$diff_output" ]; then
       echo -e "${GREEN}PASS${NC}: test ${n}"
       ((pass_count++))
     else
       echo -e "${RED}FAIL${NC}: test ${n}"
       echo "$diff_output"
       ((fail_count++))
     fi
   done

   echo "-----------------------------"
   echo "Results for ${TEST_FOLDER}: ${pass_count} passed, ${fail_count} failed"

   # non-zero exit if any test failed — useful for CI / scripting
   [ "$fail_count" -eq 0 ]
 }