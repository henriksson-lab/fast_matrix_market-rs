use fast_matrix_market::header::read_header_line_166;
use fast_matrix_market::read_body::{
    line_counts, read_chunk_matrix_coordinate_line_213, read_matrix_market_body_line_562,
    read_real_or_complex_line_193,
};
use fast_matrix_market::read_body_threads::{count_chunk_lines_line_24, read_body_threads_line_33};
use fast_matrix_market::types::{
    field_type, generalize_coordinate_diagnonal_values_type, matrix_market_header_line_48,
    out_of_range_behavior, read_options, symmetry_type,
};
use std::io::Cursor;

fn options() -> read_options {
    read_options {
        chunk_size_bytes: 2 << 20,
        generalize_symmetry: true,
        generalize_symmetry_app: true,
        generalize_coordinate_diagnonal_values:
            generalize_coordinate_diagnonal_values_type::ExtraZeroElement,
        parallel_ok: true,
        num_threads: 0,
        float_out_of_range_behavior: out_of_range_behavior::BestMatch,
    }
}

#[test]
fn reads_coordinate_body_from_real_fixture() {
    let mut input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/eye3.mtx"
    ));
    let mut header = matrix_market_header_line_48();
    read_header_line_166(&mut input, &mut header).unwrap();

    let entries =
        read_matrix_market_body_line_562::<Vec<f64>>(&mut input, &header, &options()).unwrap();

    assert_eq!(
        entries,
        vec![(0, 0, vec![1.0]), (1, 1, vec![1.0]), (2, 2, vec![1.0])]
    );
}

#[test]
fn reads_complex_coordinate_values() {
    let mut input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/eye3_complex.mtx"
    ));
    let mut header = matrix_market_header_line_48();
    read_header_line_166(&mut input, &mut header).unwrap();

    let entries =
        read_matrix_market_body_line_562::<Vec<f64>>(&mut input, &header, &options()).unwrap();

    assert_eq!(entries[0], (0, 0, vec![1.0, 0.0]));
    assert_eq!(entries.len() as i64, header.nnz);
}

#[test]
fn expands_symmetric_coordinate_entries() {
    let mut input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/symmetry/coordinate_symmetric_row.mtx"
    ));
    let mut header = matrix_market_header_line_48();
    read_header_line_166(&mut input, &mut header).unwrap();

    let entries =
        read_matrix_market_body_line_562::<Vec<f64>>(&mut input, &header, &options()).unwrap();

    assert!(entries.iter().any(|entry| entry.0 != entry.1));
    assert!(entries.iter().any(|entry| entry.0 == 1 && entry.1 == 2));
}

#[test]
fn append_style_symmetric_coordinate_keeps_one_diagonal_entry() {
    let mut header = matrix_market_header_line_48();
    header.nrows = 2;
    header.ncols = 2;
    header.nnz = 1;
    header.symmetry = symmetry_type::symmetric;

    let (counts, entries) = read_chunk_matrix_coordinate_line_213::<Vec<f64>>(
        "1 1 7\n",
        &header,
        line_counts {
            file_line: 0,
            element_num: 0,
        },
        &options(),
    )
    .unwrap();

    assert_eq!(counts.element_num, 1);
    assert_eq!(entries, vec![(0, 0, vec![7.0])]);
}

#[test]
fn reports_body_line_for_out_of_bounds_coordinate() {
    let mut header = matrix_market_header_line_48();
    header.nrows = 2;
    header.ncols = 2;
    header.nnz = 1;

    let err = read_chunk_matrix_coordinate_line_213::<Vec<f64>>(
        "3 1 5\n",
        &header,
        line_counts {
            file_line: 3,
            element_num: 0,
        },
        &options(),
    )
    .unwrap_err();

    assert_eq!(err.msg, "Line 4: Row index out of bounds");
}

#[test]
fn coordinate_indices_use_translated_integer_parser() {
    let mut header = matrix_market_header_line_48();
    header.nrows = 2;
    header.ncols = 2;
    header.nnz = 1;

    let (counts, entries) = read_chunk_matrix_coordinate_line_213::<Vec<f64>>(
        "+1 2 5\n",
        &header,
        line_counts {
            file_line: 0,
            element_num: 0,
        },
        &options(),
    )
    .unwrap();
    assert_eq!(counts.element_num, 1);
    assert_eq!(entries, vec![(0, 1, vec![5.0])]);

    let err = read_chunk_matrix_coordinate_line_213::<Vec<f64>>(
        "1junk 2 5\n",
        &header,
        line_counts {
            file_line: 0,
            element_num: 0,
        },
        &options(),
    )
    .unwrap_err();
    assert_eq!(err.msg, "Line 1: Invalid integer value.");
}

#[test]
fn threaded_reader_shim_matches_sequential_entries() {
    let mut input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/eye3.mtx"
    ));
    let mut header = matrix_market_header_line_48();
    read_header_line_166(&mut input, &mut header).unwrap();

    let (_counts, entries) =
        read_body_threads_line_33::<Vec<f64>>(&mut input, &header, &options()).unwrap();

    assert_eq!(
        entries,
        vec![(0, 0, vec![1.0]), (1, 1, vec![1.0]), (2, 2, vec![1.0])]
    );
}

#[test]
fn threaded_reader_shim_reads_typed_complex_values() {
    let mut input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/eye3_complex.mtx"
    ));
    let mut header = matrix_market_header_line_48();
    read_header_line_166(&mut input, &mut header).unwrap();

    let (_counts, entries) =
        read_body_threads_line_33::<(f64, f64)>(&mut input, &header, &options()).unwrap();

    assert_eq!(entries[0], (0, 0, (1.0, 0.0)));
    assert_eq!(entries.len() as i64, header.nnz);
}

#[test]
fn read_real_or_complex_accepts_template_values() {
    let mut header = matrix_market_header_line_48();
    header.field = field_type::real;
    assert_eq!(
        read_real_or_complex_line_193::<f64>(&["2.5"], &header, &options()).unwrap(),
        2.5
    );
    assert_eq!(
        read_real_or_complex_line_193::<f64>(&["2.5abc"], &header, &options()).unwrap(),
        2.5
    );
    assert_eq!(
        read_real_or_complex_line_193::<(f64, f64)>(&["2.5"], &header, &options()).unwrap(),
        (2.5, 0.0)
    );

    header.field = field_type::complex;
    assert_eq!(
        read_real_or_complex_line_193::<(f64, f64)>(&["2.5", "-3.5"], &header, &options()).unwrap(),
        (2.5, -3.5)
    );
    assert!(read_real_or_complex_line_193::<f64>(&["2.5", "-3.5"], &header, &options()).is_err());

    header.field = field_type::pattern;
    assert_eq!(
        read_real_or_complex_line_193::<()>(&[], &header, &options()).unwrap(),
        ()
    );
}

#[test]
fn read_real_or_complex_honors_out_of_range_options() {
    let mut header = matrix_market_header_line_48();
    header.field = field_type::real;

    let mut best_match = options();
    best_match.float_out_of_range_behavior = out_of_range_behavior::BestMatch;
    assert!(
        read_real_or_complex_line_193::<f64>(&["1e9999"], &header, &best_match)
            .unwrap()
            .is_infinite()
    );

    let mut throw = options();
    throw.float_out_of_range_behavior = out_of_range_behavior::ThrowOutOfRange;
    let err = read_real_or_complex_line_193::<f64>(&["1e9999"], &header, &throw).unwrap_err();
    assert_eq!(err.msg, "Floating-point value out of range.");
}

#[test]
fn count_chunk_lines_wraps_chunk_and_counts() {
    let result = count_chunk_lines_line_24("1 1 1\n\n2 2 1\n".to_string());

    assert_eq!(result.counts.file_line, 3);
    assert_eq!(result.counts.element_num, 2);
    assert_eq!(result.chunk, "1 1 1\n\n2 2 1\n");
}
