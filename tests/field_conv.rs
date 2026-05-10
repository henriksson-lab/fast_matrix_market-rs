use fast_matrix_market::app_user_type_string;
use fast_matrix_market::field_conv::{
    bump_to_next_line_line_48, complex_conjugate_line_347, read_float_fallback_line_196,
    read_float_fallback_line_216, read_int_line_140, read_value_line_317, read_value_line_321,
    read_value_line_329, read_value_line_334, skip_spaces_and_newlines_line_38,
    skip_spaces_line_34, value_to_string_fallback_line_404, value_to_string_line_391,
    value_to_string_line_396, value_to_string_line_601, value_to_string_line_630,
};

#[test]
fn whitespace_scanners_match_c_pointer_behavior() {
    assert_eq!(skip_spaces_line_34(" \t\rabc", 0), 3);
    assert_eq!(skip_spaces_and_newlines_line_38(" \n\t\nx", 0), 4);
    assert_eq!(bump_to_next_line_line_48("abc\ndef", 0), 4);
    assert_eq!(bump_to_next_line_line_48("abc", 0), 3);
}

#[test]
fn parses_integer_bool_float_and_complex_values() {
    assert_eq!(read_int_line_140("-42 rest", 0).unwrap(), (3, -42));
    assert_eq!(read_value_line_317("123 ", 0).unwrap(), (3, 123));
    assert_eq!(read_value_line_321("0 ", 0).unwrap(), (1, false));
    assert_eq!(read_value_line_321("2.5 ", 0).unwrap(), (3, true));
    assert_eq!(read_value_line_329("-1.25 x", 0).unwrap(), (5, -1.25));
    assert_eq!(
        read_float_fallback_line_196("1.25abc", 0).unwrap(),
        (4, 1.25)
    );
    assert_eq!(
        read_float_fallback_line_196(" \t1.25abc", 0).unwrap(),
        (6, 1.25)
    );
    assert_eq!(
        read_float_fallback_line_196("1e nope", 0).unwrap(),
        (1, 1.0)
    );
    let (next, nan_value) = read_float_fallback_line_196("nan(payload) rest", 0).unwrap();
    assert_eq!(next, 12);
    assert!(nan_value.is_nan());
    assert!(read_float_fallback_line_196(". nope", 0).is_err());
    assert_eq!(
        read_float_fallback_line_216("1e39", 0).unwrap_err(),
        "Floating-point value out of range."
    );
    assert_eq!(
        read_float_fallback_line_216("1e-50", 0).unwrap_err(),
        "Floating-point value out of range."
    );
    assert!(read_float_fallback_line_216("inf", 0)
        .unwrap()
        .1
        .is_infinite());
    assert_eq!(
        read_value_line_334("1.5 -2.25", 0).unwrap(),
        (9, (1.5, -2.25))
    );
}

#[test]
fn converts_values_to_matrix_market_strings() {
    assert_eq!(value_to_string_line_391(true), "1");
    assert_eq!(value_to_string_line_391(false), "0");
    assert_eq!(value_to_string_line_396(17), "17");
    assert_eq!(value_to_string_line_630((1.0, -2.5), -1), "1 -2.5");
    assert_eq!(value_to_string_fallback_line_404(1.5, -1), "1.500000");
    assert_eq!(value_to_string_fallback_line_404(f64::INFINITY, -1), "inf");
    assert_eq!(
        value_to_string_fallback_line_404(f64::NEG_INFINITY, -1),
        "-inf"
    );
    assert_eq!(value_to_string_fallback_line_404(f64::NAN, -1), "nan");
    assert_eq!(value_to_string_line_601(123.456, 4), "123.5");
    assert_eq!(value_to_string_line_601(0.00123456, 4), "0.001235");
    assert_eq!(value_to_string_line_601(123456.0, 4), "1.235e+5");
    assert_eq!(complex_conjugate_line_347((1.0, -2.5)), (1.0, 2.5));
}

#[test]
fn reads_and_writes_user_string_values() {
    assert_eq!(
        app_user_type_string::read_value_line_38("abc def\nnext", 0),
        (7, "abc def".to_string())
    );
    assert_eq!(app_user_type_string::negate_line_57("abc"), "-abc");
    assert_eq!(app_user_type_string::pattern_default_value_line_64(), "");
    assert_eq!(
        app_user_type_string::value_to_string_line_83("abc", -1),
        "abc"
    );
}
