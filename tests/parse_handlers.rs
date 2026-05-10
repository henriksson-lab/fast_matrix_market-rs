use fast_matrix_market::parse_handlers as ph;
use fast_matrix_market::read_body as rb;
use fast_matrix_market::types::storage_order;

#[test]
fn tuple_and_triplet_handlers_write_at_offsets() {
    let mut tuple = ph::tuple_parse_handler_line_50(1);
    ph::handle_line_52(&mut tuple, 2, 3, vec![4.0]);
    assert_eq!(tuple.tuples[1], (2, 3, vec![4.0]));
    assert_eq!(ph::get_chunk_handler_line_57(&tuple, 4).iter, 5);

    let mut triplet = ph::triplet_parse_handler_line_76(0, 0, 0);
    ph::handle_line_81(&mut triplet, 1, 2, vec![3.0]);
    ph::handle_line_91(&mut triplet, 4, 5);
    assert_eq!(triplet.row_values, vec![1, 4]);
    assert_eq!(triplet.col_values, vec![2, 5]);
    assert_eq!(triplet.entry_values, vec![vec![3.0], vec![]]);
    assert_eq!(ph::get_chunk_handler_line_100(&triplet, 7).rows, 7);
}

#[test]
fn tuple_and_triplet_handlers_accept_template_values() {
    let mut tuple = ph::tuple_parse_handler_line_50::<(f64, f64)>(0);
    ph::handle_line_52(&mut tuple, 2, 3, (4.0, -4.0));
    assert_eq!(tuple.tuples[0], (2, 3, (4.0, -4.0)));

    let mut triplet = ph::triplet_parse_handler_line_76::<f64>(0, 0, 0);
    ph::handle_line_81(&mut triplet, 1, 2, 3.5);
    ph::handle_line_91(&mut triplet, 4, 5);
    assert_eq!(triplet.row_values, vec![1, 4]);
    assert_eq!(triplet.col_values, vec![2, 5]);
    assert_eq!(triplet.entry_values, vec![3.5, 0.0]);
}

#[test]
fn pattern_calling_and_doublet_handlers_match_upstream_indexing() {
    let mut pattern = ph::triplet_pattern_parse_handler_line_126(0, 0);
    ph::handle_line_130(&mut pattern, 8, 9);
    assert_eq!(pattern.row_values, vec![8]);
    assert_eq!(pattern.col_values, vec![9]);
    assert_eq!(ph::get_chunk_handler_line_138(&pattern, 3).cols, 3);

    let mut calling =
        ph::triplet_calling_parse_handler_line_160(Vec::new(), Vec::new(), Vec::new(), 2);
    ph::handle_line_165(&mut calling, 10, 11, vec![12.0]);
    assert_eq!(calling.row_values[2], 10);
    assert_eq!(calling.col_values[2], 11);
    assert_eq!(calling.entry_values[2], vec![12.0]);

    let mut doublet = ph::doublet_parse_handler_line_195(0, 0);
    ph::handle_line_199(&mut doublet, 0, 4, vec![5.0]);
    assert_eq!(doublet.index_values, vec![4]);
    assert_eq!(doublet.entry_values, vec![vec![5.0]]);
    assert_eq!(ph::get_chunk_handler_line_207(&doublet, 6).index, 6);
}

#[test]
fn calling_and_doublet_handlers_accept_template_values() {
    let mut calling =
        ph::triplet_calling_parse_handler_line_160(Vec::new(), Vec::new(), Vec::new(), 1);
    ph::handle_line_165(&mut calling, 10, 11, (12.0_f64, -12.0_f64));
    assert_eq!(calling.row_values[1], 10);
    assert_eq!(calling.col_values[1], 11);
    assert_eq!(calling.entry_values[1], (12.0, -12.0));

    let mut doublet = ph::doublet_parse_handler_line_195::<f64>(0, 0);
    ph::handle_line_199(&mut doublet, 0, 4, 5.5);
    assert_eq!(doublet.index_values, vec![4]);
    assert_eq!(doublet.entry_values, vec![5.5]);
}

#[test]
fn dense_handlers_add_values_with_storage_order() {
    let mut dense2d = ph::dense_2d_call_adding_parse_handler_line_229(2, 3);
    ph::handle_line_231(&mut dense2d, 1, 2, vec![1.5]);
    ph::handle_line_231(&mut dense2d, 1, 2, vec![2.5]);
    assert_eq!(dense2d.values[5], vec![4.0]);
    assert_eq!(
        ph::get_chunk_handler_line_235(&dense2d, 100).values[5],
        vec![4.0]
    );

    let mut dense = ph::dense_adding_parse_handler_line_253(
        vec![Vec::new(); 6],
        storage_order::col_major,
        2,
        3,
    );
    ph::handle_line_256(&mut dense, 1, 2, vec![7.0]);
    assert_eq!(dense.values[5], vec![7.0]);
    assert_eq!(
        ph::get_chunk_handler_line_266(&dense, 100).values[5],
        vec![7.0]
    );
}

#[test]
fn dense_handlers_accept_template_values() {
    let mut dense2d = ph::dense_2d_call_adding_parse_handler_line_229::<f64>(2, 3);
    ph::handle_line_231(&mut dense2d, 1, 2, 1.5);
    ph::handle_line_231(&mut dense2d, 1, 2, 2.5);
    assert_eq!(dense2d.values[5], 4.0);

    let mut dense = ph::dense_adding_parse_handler_line_253(
        vec![(0.0_f64, 0.0_f64); 6],
        storage_order::col_major,
        2,
        3,
    );
    ph::handle_line_256(&mut dense, 1, 2, (7.0, -1.0));
    ph::handle_line_256(&mut dense, 1, 2, (0.5, -2.0));
    assert_eq!(dense.values[5], (7.5, -3.0));
}

#[test]
fn read_body_adapters_forward_pattern_and_complex_values() {
    let mut pattern = rb::pattern_parse_adapter_line_32(Vec::new(), vec![1.0]);
    rb::handle_line_35(&mut pattern, 0, 1);
    rb::handle_line_39(&mut pattern, 2, 3, vec![4.0]);
    assert_eq!(pattern.handler, vec![(0, 1, vec![1.0]), (2, 3, vec![4.0])]);
    assert_eq!(
        rb::get_chunk_handler_line_43(&pattern, 1).handler,
        vec![(2, 3, vec![4.0])]
    );

    let mut complex = rb::complex_parse_adapter_line_67(Vec::new());
    rb::handle_line_69(&mut complex, 4, 5);
    rb::handle_line_73(&mut complex, 6, 7, 8.0);
    assert_eq!(
        complex.handler,
        vec![(4, 5, vec![]), (6, 7, vec![8.0, 0.0])]
    );
    assert_eq!(
        rb::get_chunk_handler_line_77(&complex, 1).handler,
        vec![(6, 7, vec![8.0, 0.0])]
    );
}

#[test]
fn read_body_adapters_accept_template_values() {
    let mut pattern = rb::pattern_parse_adapter_line_32(Vec::new(), 1.5_f64);
    rb::handle_line_35(&mut pattern, 0, 1);
    rb::handle_line_39(&mut pattern, 2, 3, 4.5);
    assert_eq!(pattern.handler, vec![(0, 1, 1.5), (2, 3, 4.5)]);
    assert_eq!(
        rb::get_chunk_handler_line_43(&pattern, 1).handler,
        vec![(2, 3, 4.5)]
    );

    let mut complex = rb::complex_parse_adapter_line_67::<(f64, f64)>(Vec::new());
    rb::handle_line_69(&mut complex, 4, 5);
    rb::handle_line_73(&mut complex, 6, 7, 8.0);
    assert_eq!(
        complex.handler,
        vec![(4, 5, (1.0, 0.0)), (6, 7, (8.0, 0.0))]
    );
    assert_eq!(
        rb::get_chunk_handler_line_77(&complex, 1).handler,
        vec![(6, 7, (8.0, 0.0))]
    );
}
