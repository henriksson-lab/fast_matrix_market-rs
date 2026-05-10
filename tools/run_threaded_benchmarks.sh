#!/usr/bin/env bash
set -euo pipefail

iterations="${1:-5}"
threads="${2:-1 10}"
fixtures_file="${3:-tools/benchmark_fixtures.txt}"

cargo build --release --example bench_read >/dev/null
g++ -O3 -std=c++17 -pthread -I fast_matrix_market/include \
    tools/bench_read_cpp.cpp -o .tmp/bench_read_cpp

while IFS= read -r fixture; do
    case "$fixture" in
        ""|\#*) continue ;;
    esac
    if [[ ! -f "$fixture" ]]; then
        printf 'skip missing file=%s\n' "$fixture"
        continue
    fi
    for thread_count in $threads; do
        target/release/examples/bench_read "$fixture" "$iterations" "$thread_count"
        .tmp/bench_read_cpp "$fixture" "$iterations" "$thread_count"
    done
done < "$fixtures_file"
