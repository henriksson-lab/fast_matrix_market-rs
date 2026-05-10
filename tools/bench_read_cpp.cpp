#include <chrono>
#include <complex>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include "fast_matrix_market/app/triplet.hpp"

int main(int argc, char** argv) {
    if (argc != 3 && argc != 4) {
        std::cerr << "usage: bench_read_cpp <matrix.mtx> <iterations> [threads]\n";
        return 2;
    }

    const std::string path = argv[1];
    const int iterations = std::atoi(argv[2]);
    const int threads = argc == 4 ? std::atoi(argv[3]) : 1;
    fast_matrix_market::read_options options;
    options.chunk_size_bytes = 1 << 20;
    options.generalize_symmetry = true;
    options.generalize_symmetry_app = true;
    options.parallel_ok = true;
    options.num_threads = threads;

    fast_matrix_market::matrix_market_header probe_header;
    {
        std::ifstream input(path);
        if (!input) {
            std::cerr << "open matrix failed: " << path << "\n";
            return 1;
        }
        fast_matrix_market::read_header(input, probe_header);
    }

    auto start = std::chrono::steady_clock::now();
    std::size_t checksum = 0;
    int64_t dims_rows = 0;
    int64_t dims_cols = 0;

    for (int i = 0; i < iterations; ++i) {
        std::ifstream input(path);
        if (!input) {
            std::cerr << "open matrix failed: " << path << "\n";
            return 1;
        }
        std::vector<int64_t> rows;
        std::vector<int64_t> cols;
        fast_matrix_market::matrix_market_header header;
        std::size_t value_count = 0;
        if (probe_header.field == fast_matrix_market::complex) {
            std::vector<std::complex<double>> values;
            fast_matrix_market::read_matrix_market_triplet(
                    input, header, rows, cols, values, options);
            value_count = values.size();
        } else {
            std::vector<double> values;
            fast_matrix_market::read_matrix_market_triplet(
                    input, header, rows, cols, values, options);
            value_count = values.size();
        }
        dims_rows = header.nrows;
        dims_cols = header.ncols;
        checksum += rows.size() + cols.size() + value_count;
    }

    auto elapsed = std::chrono::duration<double>(std::chrono::steady_clock::now() - start).count();
    std::cout << "impl=cpp file=" << path
              << " iterations=" << iterations
              << " threads=" << threads
              << " dims=" << dims_rows << "x" << dims_cols
              << " checksum=" << checksum
              << " seconds=" << elapsed
              << "\n";
    return 0;
}
