#!/usr/bin/env bash
set -euo pipefail

iterations="${1:-5}"
threads="${2:-1 10}"
fixtures_file="${3:-tools/benchmark_fixtures.txt}"

cargo build --release --example bench_read >/dev/null
g++ -O3 -std=c++17 -pthread -I fast_matrix_market/include \
    tools/bench_read_cpp.cpp -o .tmp/bench_read_cpp

run_with_rss() {
    local stderr_file stdout rss
    stderr_file="$(mktemp .tmp/bench-rss.XXXXXX)"
    stdout="$(/usr/bin/time -f '__max_rss_kb=%M' "$@" 2>"$stderr_file")"
    rss="$(awk -F= '$1 == "__max_rss_kb" { value=$2 } END { print value }' "$stderr_file")"
    grep -v '^__max_rss_kb=' "$stderr_file" >&2 || true
    rm -f "$stderr_file"
    printf '%s max_rss_kb=%s\n' "$stdout" "$rss"
}

while IFS= read -r fixture; do
    case "$fixture" in
        ""|\#*) continue ;;
    esac
    if [[ ! -f "$fixture" ]]; then
        printf 'skip missing file=%s\n' "$fixture"
        continue
    fi
    for thread_count in $threads; do
        run_with_rss target/release/examples/bench_read "$fixture" "$iterations" "$thread_count"
        run_with_rss .tmp/bench_read_cpp "$fixture" "$iterations" "$thread_count"
    done
done < "$fixtures_file"
