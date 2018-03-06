#!/usr/bin/env bash

sh -c "cargo clean && cargo build -p konfiscator"

_rand_malloc=$(shuf -i 500-5000 -n 1)
_bin_name="./konfiscator_malloc"

_konfiscator_so_file=$(find ./target/ -maxdepth 2 -type f -name '*.so')

sh -c "gcc -g konfiscator_malloc.c -o ${_bin_name} -L./target/debug/ -lkonfiscator -Wl,-rpath ./target/debug"
sh -c "${_bin_name} ${_rand_malloc}"
