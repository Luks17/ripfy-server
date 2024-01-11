#!/bin/bash

# requires cargo-watch to run
# install with `cargo install cargo-watch` or with your package manager

while getopts ":c" opt; do
  case $opt in
    c)
      cargo watch -q -c -w tests/ -x "test -q client_mock -- --ignored --nocapture"
      ;;
    \?)
      printf "Invalid option: -$OPTARG. \nUse -c to start the mock-client\n"
      exit 1
      ;;
  esac
done

# if RUST_LOG env variable is not set, it is setted as info
if [[ -z "${RUST_LOG}" ]]; then
  export RUST_LOG="ripfy_server=info"
fi

if [ ! -f "ripfy.sqlite" ]; then
  cargo run --bin migrator
fi

cargo watch -q -c -w src/ -x run
