use fast_matrix_market::fast_matrix_market::pattern_placeholder_type;
use fast_matrix_market::types::{field_type, write_options};
use fast_matrix_market::write_body::{
    get_field_type_line_16, get_field_type_line_24, get_field_type_line_32, get_field_type_line_40,
    write_body_line_66, write_body_sequential_line_50,
};
use fast_matrix_market::write_body_threads::write_body_threads_line_20;

#[test]
fn field_type_functions_return_expected_matrix_market_fields() {
    assert_eq!(
        get_field_type_line_16::<i64>(std::ptr::null()),
        field_type::integer
    );
    assert_eq!(
        get_field_type_line_24::<f64>(std::ptr::null()),
        field_type::real
    );
    assert_eq!(
        get_field_type_line_32::<(f64, f64)>(std::ptr::null()),
        field_type::complex
    );
    assert_eq!(
        get_field_type_line_40(std::ptr::null::<pattern_placeholder_type>()),
        field_type::pattern
    );
}

#[test]
fn write_body_dispatchers_concatenate_chunks() {
    let options = write_options {
        chunk_size_values: 2 << 12,
        parallel_ok: true,
        num_threads: 0,
        precision: -1,
        always_comment: false,
        fill_header_field_type: true,
    };
    let chunks = vec!["1 1 1\n".to_string(), "2 2 1\n".to_string()];

    let mut sequential = Vec::new();
    write_body_sequential_line_50(&mut sequential, &chunks, &options).unwrap();
    assert_eq!(String::from_utf8(sequential).unwrap(), "1 1 1\n2 2 1\n");

    let mut dispatched = Vec::new();
    write_body_line_66(&mut dispatched, &chunks, &options).unwrap();
    assert_eq!(String::from_utf8(dispatched).unwrap(), "1 1 1\n2 2 1\n");

    let mut threaded = Vec::new();
    write_body_threads_line_20(&mut threaded, &chunks, &options).unwrap();
    assert_eq!(String::from_utf8(threaded).unwrap(), "1 1 1\n2 2 1\n");
}
