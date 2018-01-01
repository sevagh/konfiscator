#!/usr/bin/env bash

sh -c "cargo clean && cargo build"

_UUID=$(cat /proc/sys/kernel/random/uuid)
_mtrace_fname="./malloc_trace_${_UUID}.txt"
_mtrace_env="MALLOC_TRACE=${_mtrace_fname}"

_ld_preload_so_file=$(find ./target/ -maxdepth 2 -type f -name '*.so')
_ld_preload_env="LD_PRELOAD=${_ld_preload_so_file}"

_rand_malloc=$(shuf -i 500-5000 -n 1)

sh -c "${_ld_preload_env} ${_mtrace_env} ./malloc ${_rand_malloc}"
sh -c "mtrace ./malloc ${_mtrace_fname} ; rm -rf ${_mtrace_fname}"
