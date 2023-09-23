#!/bin/bash

# requires cargo-watch to run
# install with `cargo install cargo-watch` or with your package manager

while getopts ":sc" opt; do
  case $opt in
    s)
      cargo watch -q -c -w src/ -x run
      ;;
    c)
      cargo watch -q -c -w tests/ -x "test -q client_mock -- --nocapture"
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
