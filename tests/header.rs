use fast_matrix_market::header::{
    get_storage_nnz_line_71, read_header_line_166, strip_trailing_cr_line_60, write_header_line_278,
};
use fast_matrix_market::types::{
    field_type, format_type, generalize_coordinate_diagnonal_values_type,
    matrix_market_header_line_48, object_type, out_of_range_behavior, read_options, symmetry_type,
    write_options,
};
use std::io::Cursor;

#[test]
fn reads_coordinate_real_general_fixture_header() {
    let mut input = Cursor::new(include_str!("fixtures/upstream/tests/matrices/eye3.mtx"));
    let mut header = matrix_market_header_line_48();

    let lines = read_header_line_166(&mut input, &mut header).unwrap();

    assert_eq!(lines, 3);
    assert_eq!(header.object, object_type::matrix);
    assert_eq!(header.format, format_type::coordinate);
    assert_eq!(header.field, field_type::real);
    assert_eq!(header.symmetry, symmetry_type::general);
    assert_eq!(header.nrows, 3);
    assert_eq!(header.ncols, 3);
    assert_eq!(header.vector_length, -1);
    assert_eq!(header.nnz, 3);
    assert_eq!(header.comment, " 3-by-3 identity matrix");
    assert_eq!(header.header_line_count, 3);
}

#[test]
fn reads_array_integer_fixture_header() {
    let mut input = Cursor::new(include_str!(
        "fixtures/upstream/python/tests/matrices/matrix_array_integer_general.mtx"
    ));
    let mut header = matrix_market_header_line_48();

    read_header_line_166(&mut input, &mut header).unwrap();

    assert_eq!(header.object, object_type::matrix);
    assert_eq!(header.format, format_type::array);
    assert_eq!(header.field, field_type::integer);
    assert_eq!(header.symmetry, symmetry_type::general);
    assert_eq!(header.nnz, header.nrows * header.ncols);
}

#[test]
fn header_dimensions_use_translated_integer_parser() {
    let mut input = Cursor::new("%%MatrixMarket matrix coordinate real general\n+2 3 4\n");
    let mut header = matrix_market_header_line_48();

    read_header_line_166(&mut input, &mut header).unwrap();

    assert_eq!((header.nrows, header.ncols, header.nnz), (2, 3, 4));

    let mut input = Cursor::new("%%MatrixMarket matrix coordinate real general\n2junk 3 4\n");
    let mut header = matrix_market_header_line_48();
    let err = read_header_line_166(&mut input, &mut header).unwrap_err();
    assert_eq!(err.msg, "Line 2: Header dimension line not of length 2");
}

#[test]
fn rejects_bad_banner_with_line_number() {
    let mut input = Cursor::new(include_str!(
        "fixtures/upstream/tests/matrices/invalid/invalid_bad_banner.mtx"
    ));
    let mut header = matrix_market_header_line_48();

    let err = read_header_line_166(&mut input, &mut header).unwrap_err();

    assert_eq!(err.msg, "Line 1: Not a Matrix Market file. Missing banner.");
}

#[test]
fn rejects_missing_banner_fields_like_upstream() {
    let mut input = Cursor::new("%%MatrixMarket matrix coordinate real\n3 3 1\n");
    let mut header = matrix_market_header_line_48();

    let err = read_header_line_166(&mut input, &mut header).unwrap_err();

    assert_eq!(err.msg, "Line 1: Invalid MatrixMarket header element: ");
}

#[test]
fn rejects_extra_dimension_tokens_like_upstream() {
    let mut input = Cursor::new("%%MatrixMarket matrix coordinate real general\n3 3 1 extra\n");
    let mut header = matrix_market_header_line_48();

    let err = read_header_line_166(&mut input, &mut header).unwrap_err();

    assert_eq!(err.msg, "Line 2: Header dimension line not of length 3");
}

#[test]
fn writes_header_matching_upstream_comment_format() {
    let mut input = Cursor::new(include_str!("fixtures/upstream/tests/matrices/eye3.mtx"));
    let mut header = matrix_market_header_line_48();
    read_header_line_166(&mut input, &mut header).unwrap();

    let mut output = Vec::new();
    write_header_line_278(
        &mut output,
        &header,
        write_options {
            chunk_size_values: 2 << 12,
            parallel_ok: true,
            num_threads: 0,
            precision: -1,
            always_comment: false,
            fill_header_field_type: true,
        },
    )
    .unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        "%%MatrixMarket matrix coordinate real general\n% 3-by-3 identity matrix\n3 3 3\n"
    );
}

#[test]
fn strips_only_trailing_carriage_return() {
    let mut line = String::from("abc\r");

    strip_trailing_cr_line_60(&mut line);

    assert_eq!(line, "abc");
}

#[test]
fn storage_nnz_matches_symmetric_coordinate_generalization() {
    let mut header = matrix_market_header_line_48();
    header.format = format_type::coordinate;
    header.symmetry = symmetry_type::symmetric;
    header.nnz = 4;

    let mut options = read_options {
        chunk_size_bytes: 2 << 20,
        generalize_symmetry: true,
        generalize_symmetry_app: true,
        generalize_coordinate_diagnonal_values:
            generalize_coordinate_diagnonal_values_type::ExtraZeroElement,
        parallel_ok: true,
        num_threads: 0,
        float_out_of_range_behavior: out_of_range_behavior::BestMatch,
    };

    assert_eq!(
        get_storage_nnz_line_71(&header, options.clone()).unwrap(),
        8
    );

    options.generalize_symmetry = false;
    assert_eq!(get_storage_nnz_line_71(&header, options).unwrap(), 4);
}
