#!/bin/bash

# if RUST_LOG env variable is not set, it is setted as info
if [[ -z "${RUST_LOG}" ]]; then
  export RUST_LOG="ripfy_server=info"
fi

# requires cargo-watch to run
# install with `cargo install cargo-watch` or with your package manager

while getopts ":sct" opt; do
  case $opt in
    s)
      cargo watch -q -c -w src/ -x run
      ;;
    c)
      cargo watch -q -c -w tests/ -x "test -q client_mock -- --ignored --nocapture"
      ;;
    t)
      cargo watch -q -c -w tests/ -x test
      ;;
    \?)
      printf "Invalid option: -$OPTARG. \nUse -c to watch the mock-client or -s to watch server instead!\n"
      exit 1
      ;;
    :)
      printf "Option -$OPTARG requires an argument.\n"
      exit 1
      ;;
  esac
done
