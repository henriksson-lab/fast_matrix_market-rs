use fast_matrix_market::app_triplet::read_matrix_market_triplet_line_126;
use fast_matrix_market::header::read_header_line_166;
use fast_matrix_market::types::{
    field_type, generalize_coordinate_diagnonal_values_type, matrix_market_header_line_48,
    out_of_range_behavior, read_options,
};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        eprintln!("usage: bench_read <matrix.mtx> <iterations> [threads]");
        std::process::exit(2);
    }
    let path = &args[1];
    let iterations: usize = args[2].parse().expect("iterations must be a number");
    let threads: i64 = args
        .get(3)
        .map(|value| value.parse().expect("threads must be a number"))
        .unwrap_or(1);
    let options = read_options {
        chunk_size_bytes: 1 << 20,
        generalize_symmetry: true,
        generalize_symmetry_app: true,
        generalize_coordinate_diagnonal_values:
            generalize_coordinate_diagnonal_values_type::ExtraZeroElement,
        parallel_ok: true,
        num_threads: threads,
        float_out_of_range_behavior: out_of_range_behavior::BestMatch,
    };

    let mut header = matrix_market_header_line_48();
    {
        let file = File::open(path).expect("open matrix");
        let mut reader = BufReader::new(file);
        read_header_line_166(&mut reader, &mut header).expect("read header");
    }

    let start = Instant::now();
    let mut checksum = 0usize;
    let mut dims = (0i64, 0i64);
    for _ in 0..iterations {
        let file = File::open(path).expect("open matrix");
        let mut reader = BufReader::new(file);
        let (nrows, ncols, rows_len, cols_len, value_count) = match header.field {
            field_type::pattern => {
                let (nrows, ncols, rows, cols, values) =
                    read_matrix_market_triplet_line_126::<()>(&mut reader, &options)
                        .expect("read matrix");
                (nrows, ncols, rows.len(), cols.len(), values.len())
            }
            field_type::complex => {
                let (nrows, ncols, rows, cols, values) =
                    read_matrix_market_triplet_line_126::<(f64, f64)>(&mut reader, &options)
                        .expect("read matrix");
                (nrows, ncols, rows.len(), cols.len(), values.len())
            }
            _ => {
                let (nrows, ncols, rows, cols, values) =
                    read_matrix_market_triplet_line_126::<f64>(&mut reader, &options)
                        .expect("read matrix");
                (nrows, ncols, rows.len(), cols.len(), values.len())
            }
        };
        dims = (nrows, ncols);
        checksum = checksum
            .wrapping_add(rows_len)
            .wrapping_add(cols_len)
            .wrapping_add(value_count);
    }
    let elapsed = start.elapsed();
    println!(
        "impl=rust file={} iterations={} threads={} dims={}x{} checksum={} seconds={:.9}",
        path,
        iterations,
        threads,
        dims.0,
        dims.1,
        checksum,
        elapsed.as_secs_f64()
    );
}
