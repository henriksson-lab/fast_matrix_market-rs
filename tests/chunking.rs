use fast_matrix_market::chunking::{
    count_lines_line_66, get_next_chunk_line_51, is_all_spaces_line_59,
};
use fast_matrix_market::types::{
    generalize_coordinate_diagnonal_values_type, out_of_range_behavior, read_options,
};
use std::io::Cursor;

#[test]
fn counts_total_and_empty_lines_like_upstream() {
    assert_eq!(count_lines_line_66(""), (1, 1));
    assert_eq!(count_lines_line_66("abc"), (1, 0));
    assert_eq!(count_lines_line_66("abc\n \t\r\nlast"), (3, 1));
    assert_eq!(count_lines_line_66("abc\n"), (1, 0));
    assert!(is_all_spaces_line_59(" \t\r"));
    assert!(!is_all_spaces_line_59(" \nx"));
}

#[test]
fn chunk_extends_to_end_of_line() {
    let options = read_options {
        chunk_size_bytes: 4100,
        generalize_symmetry: true,
        generalize_symmetry_app: true,
        generalize_coordinate_diagnonal_values:
            generalize_coordinate_diagnonal_values_type::ExtraZeroElement,
        parallel_ok: true,
        num_threads: 0,
        float_out_of_range_behavior: out_of_range_behavior::BestMatch,
    };
    let mut input = Cursor::new("abcd\nefgh\n");

    let chunk = get_next_chunk_line_51(&mut input, &options).unwrap();

    assert_eq!(chunk, "abcd\n");
}
