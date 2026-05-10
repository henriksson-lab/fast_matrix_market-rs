use fast_matrix_market::app_array::{
    read_matrix_market_array_line_66, write_matrix_market_array_line_94,
};
use fast_matrix_market::app_triplet::{
    generalize_symmetry_triplet_line_45, read_matrix_market_triplet_line_126,
    write_matrix_market_csc_line_173, write_matrix_market_triplet_line_140,
};
use fast_matrix_market::types::{
    field_type, format_type, generalize_coordinate_diagnonal_values_type,
    matrix_market_header_line_50, out_of_range_behavior, read_options, storage_order,
    symmetry_type, write_options,
};
use std::io::Cursor;

fn read_opts() -> read_options {
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

fn write_opts() -> write_options {
    write_options {
        chunk_size_values: 2,
        parallel_ok: true,
        num_threads: 0,
        precision: -1,
        always_comment: false,
        fill_header_field_type: true,
    }
}

fn write_opts_with_precision(precision: i64) -> write_options {
    let mut options = write_opts();
    options.precision = precision;
    options
}

#[test]
fn array_binding_reads_dense_fixture() {
    let mut input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/eye3_array.mtx"
    ));

    let (nrows, ncols, values) = read_matrix_market_array_line_66::<Vec<f64>>(
        &mut input,
        storage_order::row_major,
        &read_opts(),
    )
    .unwrap();

    assert_eq!((nrows, ncols), (3, 3));
    assert_eq!(values[0], vec![1.0]);
    assert_eq!(values[4], vec![1.0]);
    assert_eq!(values[8], vec![1.0]);
}

#[test]
fn array_binding_reads_complex_dense_fixture() {
    let mut input = Cursor::new(include_str!(
        "../fast_matrix_market/python/tests/matrices/matrix_array_complex_general.mtx"
    ));

    let (nrows, ncols, values) = read_matrix_market_array_line_66::<Vec<f64>>(
        &mut input,
        storage_order::row_major,
        &read_opts(),
    )
    .unwrap();

    assert_eq!((nrows, ncols), (3, 2));
    assert_eq!(values[0], vec![1.0, 0.0]);
    assert_eq!(values[1], vec![11.0, 0.0]);
    assert_eq!(values[5], vec![33.0, 0.0]);
}

#[test]
fn array_binding_reads_typed_dense_values() {
    let mut real_input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/eye3_array.mtx"
    ));

    let (nrows, ncols, values) = read_matrix_market_array_line_66::<f64>(
        &mut real_input,
        storage_order::row_major,
        &read_opts(),
    )
    .unwrap();

    assert_eq!((nrows, ncols), (3, 3));
    assert_eq!(values[0], 1.0);
    assert_eq!(values[4], 1.0);
    assert_eq!(values[8], 1.0);

    let mut complex_input = Cursor::new(include_str!(
        "../fast_matrix_market/python/tests/matrices/matrix_array_complex_general.mtx"
    ));

    let (nrows, ncols, values) = read_matrix_market_array_line_66::<(f64, f64)>(
        &mut complex_input,
        storage_order::row_major,
        &read_opts(),
    )
    .unwrap();

    assert_eq!((nrows, ncols), (3, 2));
    assert_eq!(values[0], (1.0, 0.0));
    assert_eq!(values[1], (11.0, 0.0));
    assert_eq!(values[5], (33.0, 0.0));
}

#[test]
fn array_binding_specialized_reader_uses_shared_float_parser() {
    let mut input =
        Cursor::new("%%MatrixMarket matrix array real general\n1 2\n2.5suffix\n1e9999\n");
    let (nrows, ncols, values) =
        read_matrix_market_array_line_66::<f64>(&mut input, storage_order::row_major, &read_opts())
            .unwrap();

    assert_eq!((nrows, ncols), (1, 2));
    assert_eq!(values[0], 2.5);
    assert!(values[1].is_infinite());

    let mut options = read_opts();
    options.float_out_of_range_behavior = out_of_range_behavior::ThrowOutOfRange;
    let mut input = Cursor::new("%%MatrixMarket matrix array real general\n1 1\n1e9999\n");
    let err =
        read_matrix_market_array_line_66::<f64>(&mut input, storage_order::row_major, &options)
            .unwrap_err();
    assert_eq!(err.msg, "Line 3: Floating-point value out of range.");
}

#[test]
fn triplet_binding_reads_coordinate_fixture() {
    let mut input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/eye3.mtx"
    ));

    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<f64>(&mut input, &read_opts()).unwrap();

    assert_eq!((nrows, ncols), (3, 3));
    assert_eq!(rows, vec![0, 1, 2]);
    assert_eq!(cols, vec![0, 1, 2]);
    assert_eq!(values, vec![1.0, 1.0, 1.0]);
}

#[test]
fn triplet_binding_fast_reader_uses_shared_parsers() {
    let mut input =
        Cursor::new("%%MatrixMarket matrix coordinate real general\n2 2 1\n+1 2 2.5suffix\n");
    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<f64>(&mut input, &read_opts()).unwrap();

    assert_eq!((nrows, ncols), (2, 2));
    assert_eq!(rows, vec![0]);
    assert_eq!(cols, vec![1]);
    assert_eq!(values, vec![2.5]);

    let mut options = read_opts();
    options.float_out_of_range_behavior = out_of_range_behavior::ThrowOutOfRange;
    let mut input =
        Cursor::new("%%MatrixMarket matrix coordinate real general\n1 1 1\n1 1 1e9999\n");
    let err = read_matrix_market_triplet_line_126::<f64>(&mut input, &options).unwrap_err();
    assert_eq!(err.msg, "Line 3: Floating-point value out of range.");
}

#[test]
fn triplet_binding_fast_reader_rejects_partial_coordinate_indices() {
    let mut input =
        Cursor::new("%%MatrixMarket matrix coordinate real general\n2 2 1\n1junk 2 5\n");

    let err = read_matrix_market_triplet_line_126::<f64>(&mut input, &read_opts()).unwrap_err();

    assert_eq!(err.msg, "Line 3: Invalid integer value.");
}

#[test]
fn triplet_binding_reads_typed_scalar_and_complex_values() {
    let mut real_input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/eye3.mtx"
    ));
    let (_nrows, _ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<f64>(&mut real_input, &read_opts()).unwrap();
    assert_eq!(rows, vec![0, 1, 2]);
    assert_eq!(cols, vec![0, 1, 2]);
    assert_eq!(values, vec![1.0, 1.0, 1.0]);

    let mut complex_input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/vector_coordinate_complex.mtx"
    ));
    let (_nrows, _ncols, _rows, _cols, values) =
        read_matrix_market_triplet_line_126::<(f64, f64)>(&mut complex_input, &read_opts())
            .unwrap();
    assert!(values.iter().any(|value| value.1 != 0.0));

    let mut pattern_input = Cursor::new(include_str!(
        "../fast_matrix_market/python/tests/matrices/matrix_coordinate_pattern_general.mtx"
    ));
    let (_nrows, _ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<()>(&mut pattern_input, &read_opts()).unwrap();
    assert_eq!(rows.len(), values.len());
    assert_eq!(cols.len(), values.len());
}

#[test]
fn triplet_binding_enforces_template_value_compatibility() {
    let mut complex_as_real = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/vector_coordinate_complex.mtx"
    ));
    let err =
        read_matrix_market_triplet_line_126::<f64>(&mut complex_as_real, &read_opts()).unwrap_err();
    assert_eq!(
        err.msg,
        "Matrix Market file has complex fields but passed data structure cannot handle complex values."
    );

    let mut pattern_input = Cursor::new(include_str!(
        "../fast_matrix_market/python/tests/matrices/matrix_coordinate_pattern_general.mtx"
    ));
    let (_nrows, _ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<()>(&mut pattern_input, &read_opts()).unwrap();
    assert_eq!(rows.len(), values.len());
    assert_eq!(cols.len(), values.len());

    let mut real_as_complex = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/eye3.mtx"
    ));
    let (_nrows, _ncols, _rows, _cols, values) =
        read_matrix_market_triplet_line_126::<(f64, f64)>(&mut real_as_complex, &read_opts())
            .unwrap();
    assert_eq!(values, vec![(1.0, 0.0), (1.0, 0.0), (1.0, 0.0)]);
}

#[test]
fn triplet_symmetry_generalization_appends_off_diagonal_mirror() {
    let mut rows = vec![2, 2, 2];
    let mut cols = vec![0, 1, 2];
    let mut values = vec![vec![10.0], vec![20.0], vec![30.0]];

    generalize_symmetry_triplet_line_45(
        &mut rows,
        &mut cols,
        &mut values,
        symmetry_type::symmetric,
    );

    assert_eq!(rows, vec![2, 2, 2, 0, 1]);
    assert_eq!(cols, vec![0, 1, 2, 2, 2]);
    assert_eq!(values[3], vec![10.0]);
}

#[test]
fn array_and_triplet_bindings_write_matrix_market_text() {
    let mut header = matrix_market_header_line_50(2, 2);
    header.field = field_type::real;

    let mut array_out = Vec::new();
    write_matrix_market_array_line_94(
        &mut array_out,
        header.clone(),
        vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0]],
        storage_order::row_major,
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(array_out).unwrap(),
        "%%MatrixMarket matrix array real general\n2 2\n1\n3\n2\n4\n"
    );

    header.format = format_type::coordinate;
    let mut triplet_out = Vec::new();
    write_matrix_market_triplet_line_140(
        &mut triplet_out,
        header.clone(),
        vec![0, 1],
        vec![0, 1],
        vec![vec![1.0], vec![4.0]],
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(triplet_out).unwrap(),
        "%%MatrixMarket matrix coordinate real general\n2 2 2\n1 1 1\n2 2 4\n"
    );
}

#[test]
fn array_binding_writes_template_value_types() {
    let header = matrix_market_header_line_50(2, 2);

    let mut real_out = Vec::new();
    write_matrix_market_array_line_94(
        &mut real_out,
        header.clone(),
        vec![1.5_f64, 2.5, 3.5, 4.5],
        storage_order::row_major,
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(real_out).unwrap(),
        "%%MatrixMarket matrix array real general\n2 2\n1.5\n3.5\n2.5\n4.5\n"
    );

    let mut complex_out = Vec::new();
    write_matrix_market_array_line_94(
        &mut complex_out,
        header,
        vec![(1.0_f64, 0.5_f64), (2.0, 1.5), (3.0, 2.5), (4.0, 3.5)],
        storage_order::row_major,
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(complex_out).unwrap(),
        "%%MatrixMarket matrix array complex general\n2 2\n1 0.5\n3 2.5\n2 1.5\n4 3.5\n"
    );
}

#[test]
fn array_binding_writes_general_precision_values() {
    let header = matrix_market_header_line_50(1, 3);
    let mut out = Vec::new();
    write_matrix_market_array_line_94(
        &mut out,
        header,
        vec![123.456_f64, 0.00123456, 123456.0],
        storage_order::row_major,
        &write_opts_with_precision(4),
    )
    .unwrap();

    assert_eq!(
        String::from_utf8(out).unwrap(),
        "%%MatrixMarket matrix array real general\n1 3\n123.5\n0.001235\n1.235e+5\n"
    );
}

#[test]
fn triplet_binding_writes_template_value_types() {
    let mut real_header = matrix_market_header_line_50(2, 2);
    let mut real_out = Vec::new();
    write_matrix_market_triplet_line_140(
        &mut real_out,
        real_header.clone(),
        vec![0, 1],
        vec![0, 1],
        vec![1.5_f64, 4.25],
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(real_out).unwrap(),
        "%%MatrixMarket matrix coordinate real general\n2 2 2\n1 1 1.5\n2 2 4.25\n"
    );

    let mut complex_out = Vec::new();
    write_matrix_market_triplet_line_140(
        &mut complex_out,
        real_header.clone(),
        vec![0],
        vec![1],
        vec![(2.0_f64, -3.5_f64)],
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(complex_out).unwrap(),
        "%%MatrixMarket matrix coordinate complex general\n2 2 1\n1 2 2 -3.5\n"
    );

    real_header.field = field_type::pattern;
    let mut pattern_out = Vec::new();
    write_matrix_market_triplet_line_140(
        &mut pattern_out,
        real_header,
        vec![0, 1],
        vec![1, 0],
        vec![(), ()],
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(pattern_out).unwrap(),
        "%%MatrixMarket matrix coordinate pattern general\n2 2 2\n1 2\n2 1\n"
    );
}

#[test]
fn csc_binding_writes_coordinate_text() {
    let mut header = matrix_market_header_line_50(2, 2);
    header.field = field_type::real;
    let mut out = Vec::new();

    write_matrix_market_csc_line_173(
        &mut out,
        header,
        vec![0, 1, 2],
        vec![0, 1],
        vec![vec![1.0], vec![4.0]],
        false,
        &write_opts(),
    )
    .unwrap();

    assert_eq!(
        String::from_utf8(out).unwrap(),
        "%%MatrixMarket matrix coordinate real general\n2 2 2\n1 1 1\n2 2 4\n"
    );
}

#[test]
fn csc_binding_writes_template_value_types() {
    let header = matrix_market_header_line_50(2, 2);

    let mut real_out = Vec::new();
    write_matrix_market_csc_line_173(
        &mut real_out,
        header.clone(),
        vec![0, 1, 2],
        vec![0, 1],
        vec![1.5_f64, 4.25],
        false,
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(real_out).unwrap(),
        "%%MatrixMarket matrix coordinate real general\n2 2 2\n1 1 1.5\n2 2 4.25\n"
    );

    let mut complex_out = Vec::new();
    write_matrix_market_csc_line_173(
        &mut complex_out,
        header.clone(),
        vec![0, 1, 1],
        vec![1],
        vec![(2.0_f64, -3.5_f64)],
        false,
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(complex_out).unwrap(),
        "%%MatrixMarket matrix coordinate complex general\n2 2 1\n2 1 2 -3.5\n"
    );

    let mut pattern_header = header;
    pattern_header.field = field_type::pattern;
    let mut pattern_out = Vec::new();
    write_matrix_market_csc_line_173(
        &mut pattern_out,
        pattern_header,
        vec![0, 1, 2],
        vec![1, 0],
        vec![(), ()],
        false,
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(pattern_out).unwrap(),
        "%%MatrixMarket matrix coordinate pattern general\n2 2 2\n2 1\n1 2\n"
    );
}
