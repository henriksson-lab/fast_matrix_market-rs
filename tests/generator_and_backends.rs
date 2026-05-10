use fast_matrix_market::app_generator::write_matrix_market_generated_triplet_line_81;
use fast_matrix_market::field_conv::{
    read_float_fast_float_line_153, read_float_from_chars_line_175, read_float_from_chars_line_257,
    read_int_fallback_line_101, read_int_fallback_line_122, read_int_fallback_line_85,
    read_int_from_chars_line_72, value_to_string_dragonbox_line_452, value_to_string_ryu_line_501,
    value_to_string_to_chars_line_537,
};
use fast_matrix_market::thirdparty_task_thread_pool::{
    std_future_r_submit_line_248, task_thread_pool_line_137, wait_for_tasks_line_291,
};
use fast_matrix_market::types::{field_type, matrix_market_header_line_50, write_options};

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

fn generated_diag(index: i64) -> (i64, i64, Vec<f64>) {
    (index, index, vec![(index + 1) as f64])
}

fn generated_diag_real(index: i64) -> (i64, i64, f64) {
    (index, index, (index + 1) as f64 + 0.5)
}

fn generated_diag_complex(index: i64) -> (i64, i64, (f64, f64)) {
    (index, index, ((index + 1) as f64, -((index + 1) as f64)))
}

fn generated_diag_pattern(index: i64) -> (i64, i64, ()) {
    (index, index, ())
}

#[test]
fn generator_binding_writes_triplets_in_chunks() {
    let mut header = matrix_market_header_line_50(3, 3);
    header.field = field_type::real;
    let mut out = Vec::new();

    write_matrix_market_generated_triplet_line_81(
        &mut out,
        header,
        3,
        generated_diag,
        &write_opts(),
    )
    .unwrap();

    assert_eq!(
        String::from_utf8(out).unwrap(),
        "%%MatrixMarket matrix coordinate real general\n3 3 3\n1 1 1\n2 2 2\n3 3 3\n"
    );
}

#[test]
fn generator_binding_writes_template_value_types() {
    let header = matrix_market_header_line_50(2, 2);

    let mut real_out = Vec::new();
    write_matrix_market_generated_triplet_line_81(
        &mut real_out,
        header.clone(),
        2,
        generated_diag_real,
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(real_out).unwrap(),
        "%%MatrixMarket matrix coordinate real general\n2 2 2\n1 1 1.5\n2 2 2.5\n"
    );

    let mut complex_out = Vec::new();
    write_matrix_market_generated_triplet_line_81(
        &mut complex_out,
        header.clone(),
        2,
        generated_diag_complex,
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(complex_out).unwrap(),
        "%%MatrixMarket matrix coordinate complex general\n2 2 2\n1 1 1 -1\n2 2 2 -2\n"
    );

    let mut pattern_out = Vec::new();
    write_matrix_market_generated_triplet_line_81(
        &mut pattern_out,
        header,
        2,
        generated_diag_pattern,
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(pattern_out).unwrap(),
        "%%MatrixMarket matrix coordinate pattern general\n2 2 2\n1 1\n2 2\n"
    );
}

#[test]
fn backend_parsers_delegate_to_scalar_parsers() {
    assert_eq!(
        read_int_from_chars_line_72("123 x", 0, 5).unwrap(),
        (3, 123)
    );
    assert_eq!(read_int_fallback_line_85("-45 x", 0, 5).unwrap(), (3, -45));
    assert_eq!(
        read_int_fallback_line_85(" \t-45 x", 0, 7).unwrap(),
        (5, -45)
    );
    assert_eq!(read_int_fallback_line_101("+45 x", 0, 5).unwrap(), (3, 45));
    assert_eq!(
        read_int_fallback_line_101(" \t+45 x", 0, 8).unwrap(),
        (5, 45)
    );
    assert_eq!(
        read_int_fallback_line_101("-1 x", 0, 4).unwrap(),
        (2, u64::MAX)
    );
    assert_eq!(read_int_fallback_line_122("77 x", 0, 4).unwrap(), (2, 77));
    assert_eq!(
        read_float_fast_float_line_153("1.25 x", 0, 6, true).unwrap(),
        (4, 1.25)
    );
    assert_eq!(
        read_float_from_chars_line_175("-2.5 x", 0, 6, true).unwrap(),
        (4, -2.5)
    );
    assert_eq!(
        read_float_from_chars_line_257("6.5 x", 0, 5, false).unwrap(),
        (3, 6.5)
    );
    assert!(read_float_fast_float_line_153("1e9999", 0, 6, false)
        .unwrap()
        .1
        .is_infinite());
    assert_eq!(
        read_float_fast_float_line_153("1e9999", 0, 6, true).unwrap_err(),
        "Floating-point value out of range."
    );
    assert_eq!(
        read_float_from_chars_line_175("1e9999", 0, 6, true).unwrap_err(),
        "Floating-point overflow"
    );
}

#[test]
fn backend_float_formatters_return_matrix_market_text_values() {
    assert_eq!(value_to_string_dragonbox_line_452(1.5), "1.5");
    assert_eq!(value_to_string_ryu_line_501(1.25, -1), "1.25");
    assert_eq!(value_to_string_to_chars_line_537(2.5, -1), "2.5");
}

#[test]
fn task_pool_submit_returns_receiver_for_result() {
    let mut pool = task_thread_pool_line_137(1);
    let rx = std_future_r_submit_line_248(&mut pool, || 42);

    wait_for_tasks_line_291(&mut pool);

    assert_eq!(rx.recv().unwrap(), 42);
}
