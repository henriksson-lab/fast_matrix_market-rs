use fast_matrix_market::app_array::read_matrix_market_array_line_66;
use fast_matrix_market::app_triplet::read_matrix_market_triplet_line_126;
use fast_matrix_market::types::{
    generalize_coordinate_diagnonal_values_type, out_of_range_behavior, read_options, storage_order,
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

fn threaded_small_chunk_opts() -> read_options {
    let mut options = read_opts();
    options.chunk_size_bytes = 4098;
    options.num_threads = 2;
    options
}

fn sorted_triplets<V: Copy + PartialOrd>(
    rows: &[i64],
    cols: &[i64],
    values: &[V],
) -> Vec<(i64, i64, V)> {
    let mut triplets: Vec<_> = rows
        .iter()
        .copied()
        .zip(cols.iter().copied())
        .zip(values.iter().copied())
        .map(|((row, col), value)| (row, col, value))
        .collect();
    triplets.sort_by(|lhs, rhs| {
        lhs.0
            .cmp(&rhs.0)
            .then(lhs.1.cmp(&rhs.1))
            .then_with(|| lhs.2.partial_cmp(&rhs.2).unwrap())
    });
    triplets
}

#[test]
fn nist_freeformat_variants_match_coordinate_entries() {
    let fixtures = [
        include_str!("fixtures/upstream/tests/matrices/nist_ex1.mtx"),
        include_str!("fixtures/upstream/tests/matrices/nist_ex1_freeformat.mtx"),
        include_str!("fixtures/upstream/tests/matrices/nist_ex1_more_freeformat.mtx"),
        include_str!(
            "fixtures/upstream/tests/matrices/permissive/windows_lineendings_nist_ex1_more_freeformat.mtx"
        ),
    ];
    let mut expected = vec![
        (0, 0, 1.0),
        (1, 1, 10.5),
        (3, 1, 250.5),
        (2, 2, 0.015),
        (0, 3, 6.0),
        (3, 3, -280.0),
        (3, 4, 33.32),
        (4, 4, 12.0),
    ];
    expected.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0).then(lhs.1.cmp(&rhs.1)));

    for fixture in fixtures {
        let mut input = Cursor::new(fixture);
        let (nrows, ncols, rows, cols, values) =
            read_matrix_market_triplet_line_126::<f64>(&mut input, &read_opts()).unwrap();

        assert_eq!((nrows, ncols), (5, 5));
        assert_eq!(sorted_triplets(&rows, &cols, &values), expected);
    }
}

#[test]
fn graph_fixture_reads_all_coordinate_edges() {
    let mut input = Cursor::new(include_str!(
        "fixtures/upstream/tests/matrices/kepner_gilbert_graph.mtx"
    ));
    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<f64>(&mut input, &read_opts()).unwrap();

    assert_eq!((nrows, ncols), (7, 7));
    assert_eq!(rows.len(), 12);
    assert_eq!(cols.len(), 12);
    assert_eq!(values, vec![1.0; 12]);
    assert!(rows
        .iter()
        .zip(&cols)
        .any(|(&row, &col)| (row, col) == (1, 0)));
    assert!(rows
        .iter()
        .zip(&cols)
        .any(|(&row, &col)| (row, col) == (2, 5)));
}

#[test]
fn permissive_banner_and_indentation_fixtures_parse() {
    let fixtures = [
        include_str!("fixtures/upstream/tests/matrices/permissive/permissive_lines_indented.mtx"),
        include_str!(
            "fixtures/upstream/tests/matrices/permissive/permissive_banner_one_percent_eye3.mtx"
        ),
        include_str!(
            "fixtures/upstream/tests/matrices/permissive/permissive_banner_leading_spaces_eye3.mtx"
        ),
    ];

    for fixture in fixtures {
        let mut input = Cursor::new(fixture);
        let (nrows, ncols, rows, cols, values) =
            read_matrix_market_triplet_line_126::<f64>(&mut input, &read_opts()).unwrap();

        assert_eq!(nrows, ncols);
        assert_eq!(rows, cols);
        assert_eq!(rows.len(), values.len());
        assert!(values.iter().all(|&value| value == 1.0));
    }
}

#[test]
fn two_column_array_fixture_respects_storage_order() {
    let fixture = include_str!("fixtures/upstream/tests/matrices/2col_array.mtx");

    let mut row_major = Cursor::new(fixture);
    let (nrows, ncols, values) = read_matrix_market_array_line_66::<f64>(
        &mut row_major,
        storage_order::row_major,
        &read_opts(),
    )
    .unwrap();
    assert_eq!((nrows, ncols), (3, 2));
    assert_eq!(values, vec![10.0, 20.0, 11.0, 21.0, 12.0, 22.0]);

    let mut col_major = Cursor::new(fixture);
    let (nrows, ncols, values) = read_matrix_market_array_line_66::<f64>(
        &mut col_major,
        storage_order::col_major,
        &read_opts(),
    )
    .unwrap();
    assert_eq!((nrows, ncols), (3, 2));
    assert_eq!(values, vec![10.0, 11.0, 12.0, 20.0, 21.0, 22.0]);
}

#[test]
fn threaded_small_chunks_match_fixture_ordering() {
    let options = threaded_small_chunk_opts();

    let mut coordinate = Cursor::new(include_str!(
        "fixtures/upstream/tests/matrices/nist_ex1.mtx"
    ));
    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<f64>(&mut coordinate, &options).unwrap();
    assert_eq!((nrows, ncols), (5, 5));
    assert_eq!(
        rows.iter()
            .copied()
            .zip(cols.iter().copied())
            .zip(values.iter().copied())
            .collect::<Vec<_>>(),
        vec![
            ((0, 0), 1.0),
            ((1, 1), 10.5),
            ((3, 1), 250.5),
            ((2, 2), 0.015),
            ((0, 3), 6.0),
            ((3, 3), -280.0),
            ((3, 4), 33.32),
            ((4, 4), 12.0),
        ]
    );

    let mut array = Cursor::new(include_str!(
        "fixtures/upstream/tests/matrices/2col_array.mtx"
    ));
    let (nrows, ncols, values) =
        read_matrix_market_array_line_66::<f64>(&mut array, storage_order::row_major, &options)
            .unwrap();
    assert_eq!((nrows, ncols), (3, 2));
    assert_eq!(values, vec![10.0, 20.0, 11.0, 21.0, 12.0, 22.0]);
}

#[test]
fn threaded_offset_write_handles_empty_lines_and_tiny_chunks() {
    let options = threaded_small_chunk_opts();
    let text = "\
%%MatrixMarket matrix coordinate real general
4 4 4
1 1 1

  2 3 2.5
\t
4 1 -3
3 4 4.25
";
    let mut input = Cursor::new(text);

    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<f64>(&mut input, &options).unwrap();

    assert_eq!((nrows, ncols), (4, 4));
    assert_eq!(rows, vec![0, 1, 3, 2]);
    assert_eq!(cols, vec![0, 2, 0, 3]);
    assert_eq!(values, vec![1.0, 2.5, -3.0, 4.25]);
}

#[test]
fn threaded_offset_write_reads_integer_coordinate_as_f64() {
    let options = threaded_small_chunk_opts();
    let text = "\
%%MatrixMarket matrix coordinate integer general
3 3 3
1 1 7
2 3 -2
3 2 0
";
    let mut input = Cursor::new(text);

    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<f64>(&mut input, &options).unwrap();

    assert_eq!((nrows, ncols), (3, 3));
    assert_eq!(rows, vec![0, 1, 2]);
    assert_eq!(cols, vec![0, 2, 1]);
    assert_eq!(values, vec![7.0, -2.0, 0.0]);
}

#[test]
fn threaded_offset_write_reads_vector_coordinate() {
    let options = threaded_small_chunk_opts();
    let text = "\
%%MatrixMarket vector coordinate real general
5 3
1 1.5
3 -2
5 4.25
";
    let mut input = Cursor::new(text);

    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<f64>(&mut input, &options).unwrap();

    assert_eq!((nrows, ncols), (5, 1));
    assert_eq!(rows, vec![0, 2, 4]);
    assert_eq!(cols, vec![0, 0, 0]);
    assert_eq!(values, vec![1.5, -2.0, 4.25]);
}

#[test]
fn threaded_offset_write_reads_pattern_coordinate() {
    let options = threaded_small_chunk_opts();
    let text = "\
%%MatrixMarket matrix coordinate pattern general
4 4 4
1 1
2 4

4 2
3 3
";
    let mut input = Cursor::new(text);

    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<()>(&mut input, &options).unwrap();

    assert_eq!((nrows, ncols), (4, 4));
    assert_eq!(rows, vec![0, 1, 3, 2]);
    assert_eq!(cols, vec![0, 3, 1, 2]);
    assert_eq!(values, vec![(), (), (), ()]);

    let mut fixture = Cursor::new(include_str!(
        "fixtures/upstream/python/tests/matrices/matrix_coordinate_pattern_general.mtx"
    ));
    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<()>(&mut fixture, &options).unwrap();
    assert_eq!((nrows, ncols), (3, 3));
    assert_eq!(rows.len(), 3);
    assert_eq!(cols.len(), 3);
    assert_eq!(values.len(), 3);
}

#[test]
fn threaded_offset_write_reads_complex_coordinate() {
    let options = threaded_small_chunk_opts();
    let text = "\
%%MatrixMarket matrix coordinate complex general
4 4 4
1 1 1.5 -0.5
2 4 -2 3

4 2 0 4.25
3 3 -7 -8
";
    let mut input = Cursor::new(text);

    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<(f64, f64)>(&mut input, &options).unwrap();

    assert_eq!((nrows, ncols), (4, 4));
    assert_eq!(rows, vec![0, 1, 3, 2]);
    assert_eq!(cols, vec![0, 3, 1, 2]);
    assert_eq!(
        values,
        vec![(1.5, -0.5), (-2.0, 3.0), (0.0, 4.25), (-7.0, -8.0)]
    );

    let mut vector = Cursor::new(
        "\
%%MatrixMarket vector coordinate complex general
5 2
1 1 2
5 -3 4
",
    );
    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<(f64, f64)>(&mut vector, &options).unwrap();
    assert_eq!((nrows, ncols), (5, 1));
    assert_eq!(rows, vec![0, 4]);
    assert_eq!(cols, vec![0, 0]);
    assert_eq!(values, vec![(1.0, 2.0), (-3.0, 4.0)]);
}

#[test]
fn threaded_offset_write_reports_malformed_rows_late() {
    let options = threaded_small_chunk_opts();
    let text = "\
%%MatrixMarket matrix coordinate real general
3 3 3
1 1 1
2 2 2
bad 3 3
";
    let mut input = Cursor::new(text);

    let err = read_matrix_market_triplet_line_126::<f64>(&mut input, &options).unwrap_err();

    assert_eq!(err.msg, "Line 5: Invalid integer value.");
}

#[test]
fn threaded_offset_write_reports_out_of_range_indices() {
    let options = threaded_small_chunk_opts();
    let text = "\
%%MatrixMarket matrix coordinate real general
3 3 2
1 1 1
4 2 2
";
    let mut input = Cursor::new(text);

    let err = read_matrix_market_triplet_line_126::<f64>(&mut input, &options).unwrap_err();

    assert_eq!(err.msg, "Line 4: Row index out of bounds");
}

#[test]
fn threaded_offset_write_reports_truncated_files() {
    let options = threaded_small_chunk_opts();
    let text = "\
%%MatrixMarket matrix coordinate real general
3 3 3
1 1 1
2 2 2
";
    let mut input = Cursor::new(text);

    let err = read_matrix_market_triplet_line_126::<f64>(&mut input, &options).unwrap_err();

    assert_eq!(err.msg, "Truncated file. Expected another 1 lines.");
}

#[test]
fn python_symmetry_coordinate_fixtures_generalize_like_app_api() {
    let mut hermitian = Cursor::new(include_str!(
        "fixtures/upstream/python/tests/matrices/matrix_coordinate_complex_hermitian.mtx"
    ));
    let (nrows, ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<(f64, f64)>(&mut hermitian, &read_opts()).unwrap();
    assert_eq!((nrows, ncols), (3, 3));
    assert_eq!(rows.len(), 5);
    assert!(rows
        .iter()
        .zip(&cols)
        .zip(&values)
        .any(|((&row, &col), &value)| (row, col, value) == (2, 0, (10.0, 1.0))));
    assert!(rows
        .iter()
        .zip(&cols)
        .zip(&values)
        .any(|((&row, &col), &value)| (row, col, value) == (0, 2, (10.0, -1.0))));
    assert!(rows
        .iter()
        .zip(&cols)
        .zip(&values)
        .any(|((&row, &col), &value)| (row, col, value) == (2, 2, (30.0, 3.0))));

    let mut skew = Cursor::new(include_str!(
        "fixtures/upstream/python/tests/matrices/matrix_coordinate_real_skew-symmetric.mtx"
    ));
    let (_nrows, _ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<f64>(&mut skew, &read_opts()).unwrap();
    assert_eq!(rows.len(), 5);
    assert!(rows
        .iter()
        .zip(&cols)
        .zip(&values)
        .any(|((&row, &col), &value)| (row, col, value) == (0, 2, -10.0)));
    assert!(rows
        .iter()
        .zip(&cols)
        .zip(&values)
        .any(|((&row, &col), &value)| (row, col, value) == (1, 2, -20.0)));
    assert!(rows
        .iter()
        .zip(&cols)
        .zip(&values)
        .any(|((&row, &col), &value)| (row, col, value) == (2, 2, 30.0)));

    let mut pattern = Cursor::new(include_str!(
        "fixtures/upstream/python/tests/matrices/matrix_coordinate_pattern_symmetric.mtx"
    ));
    let (_nrows, _ncols, rows, cols, values) =
        read_matrix_market_triplet_line_126::<()>(&mut pattern, &read_opts()).unwrap();
    assert_eq!(rows.len(), 5);
    assert_eq!(cols.len(), 5);
    assert_eq!(values.len(), 5);
    assert!(rows
        .iter()
        .zip(&cols)
        .any(|(&row, &col)| (row, col) == (2, 0)));
    assert!(rows
        .iter()
        .zip(&cols)
        .any(|(&row, &col)| (row, col) == (0, 2)));
}
