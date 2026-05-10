use fast_matrix_market::formatters::{
    array_formatter_line_314, coord_matrix_line_23, coord_matrix_line_81,
    coord_matrix_pattern_line_42, csc_formatter_line_202, dense_2d_call_formatter_line_372,
    has_next_line_134, has_next_line_221, has_next_line_317, has_next_line_375,
    line_formatter_line_20, next_chunk_line_171, next_chunk_line_282, next_chunk_line_351,
    next_chunk_line_406, operator_line_149, operator_line_238, operator_line_326,
    operator_line_384, triplet_formatter_line_120, vector_line_formatter_line_79,
};
use fast_matrix_market::types::{
    field_type, format_type, matrix_market_header_line_48, object_type, storage_order,
    symmetry_type, write_options,
};

fn options() -> write_options {
    write_options {
        chunk_size_values: 2,
        parallel_ok: true,
        num_threads: 0,
        precision: -1,
        always_comment: false,
        fill_header_field_type: true,
    }
}

#[test]
fn formats_matrix_and_vector_coordinate_lines() {
    let mut header = matrix_market_header_line_48();
    header.object = object_type::matrix;
    header.format = format_type::coordinate;
    header.field = field_type::real;
    let lf = line_formatter_line_20(header.clone(), options());

    assert_eq!(coord_matrix_line_23(&lf, 0, 1, &[2.5]), "1 2 2.5\n");
    assert_eq!(coord_matrix_pattern_line_42(0, 1), "1 2\n");

    header.object = object_type::vector;
    let vlf = vector_line_formatter_line_79(header, options());
    assert_eq!(coord_matrix_line_81(&vlf, 2, 0, &[7.0]), "3 7\n");
}

#[test]
fn triplet_formatter_chunks_rows_columns_and_values() {
    let mut header = matrix_market_header_line_48();
    header.format = format_type::coordinate;
    header.field = field_type::real;
    let lf = line_formatter_line_20(header, options());
    let mut formatter = triplet_formatter_line_120(
        lf,
        vec![0, 1, 2],
        vec![0, 1, 2],
        vec![vec![1.0], vec![2.0], vec![3.0]],
    )
    .unwrap();

    assert!(has_next_line_134(&formatter));
    let chunk = next_chunk_line_171(&mut formatter, &options());
    assert_eq!(operator_line_149(&chunk), "1 1 1\n2 2 2\n");
    assert!(has_next_line_134(&formatter));
}

#[test]
fn csc_formatter_emits_coordinate_entries() {
    let mut header = matrix_market_header_line_48();
    header.format = format_type::coordinate;
    header.field = field_type::real;
    let lf = line_formatter_line_20(header, options());
    let mut formatter = csc_formatter_line_202(
        lf,
        vec![0, 1, 3],
        vec![0, 0, 1],
        vec![vec![1.0], vec![2.0], vec![3.0]],
        false,
    )
    .unwrap();

    assert!(has_next_line_221(&formatter));
    let chunk = next_chunk_line_282(&mut formatter, &options());
    assert_eq!(operator_line_238(&chunk), "1 1 1\n1 2 2\n2 2 3\n");
}

#[test]
fn array_formatter_emits_columns_in_matrix_market_order() {
    let mut header = matrix_market_header_line_48();
    header.format = format_type::array;
    header.field = field_type::real;
    header.symmetry = symmetry_type::general;
    let lf = line_formatter_line_20(header, options());
    let mut formatter = array_formatter_line_314(
        lf,
        vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0]],
        storage_order::row_major,
        2,
        2,
    );

    assert!(has_next_line_317(&formatter));
    let chunk = next_chunk_line_351(&mut formatter, &options());
    assert_eq!(operator_line_326(&chunk), "1\n3\n");
}

#[test]
fn dense_call_formatter_accepts_template_values() {
    let mut header = matrix_market_header_line_48();
    header.format = format_type::array;
    header.field = field_type::complex;
    header.symmetry = symmetry_type::general;
    let lf = line_formatter_line_20(header, options());
    let mut formatter = dense_2d_call_formatter_line_372(
        lf,
        vec![(1.0_f64, -1.0_f64), (2.0, -2.0), (3.0, -3.0), (4.0, -4.0)],
        2,
        2,
    );

    assert!(has_next_line_375(&formatter));
    let chunk = next_chunk_line_406(&mut formatter, &options());
    assert_eq!(operator_line_384(&chunk), "1 -1\n3 -3\n2 -2\n4 -4\n");
}

#[test]
fn dense_call_formatter_indexes_partial_column_chunks_with_matrix_width() {
    let mut header = matrix_market_header_line_48();
    header.format = format_type::array;
    header.field = field_type::real;
    header.symmetry = symmetry_type::general;
    let lf = line_formatter_line_20(header, options());

    let chunk = fast_matrix_market::formatters::chunk_line_381(
        lf,
        vec![1.0_f64, 2.0, 3.0, 4.0],
        2,
        2,
        0,
        1,
    );

    assert_eq!(operator_line_384(&chunk), "1\n3\n");
}
