#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    non_camel_case_types,
    non_snake_case,
    clippy::missing_panics_doc
)]

//! Rust translation scaffold for `fast_matrix_market`.
//!
//! Each generated item records the original C++ header location. Function
//! bodies intentionally panic until the corresponding upstream logic is
//! translated bottom-up.

/// Placeholder for C++ template parameters, iterators, pointers, and dependent types.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Placeholder;

pub mod app_armadillo {
    use super::Placeholder;

    /// Stub for `read_matrix_market_arma` at `fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:17`.
    pub fn read_matrix_market_arma_line_17(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_arma (fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:17)")
    }

    /// Stub for `read_matrix_market_arma` at `fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:31`.
    pub fn read_matrix_market_arma_line_31(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_arma (fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:31)")
    }

    /// Stub for `read_matrix_market_arma` at `fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:40`.
    pub fn read_matrix_market_arma_line_40(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_arma (fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:40)")
    }

    /// Stub for `read_matrix_market_arma` at `fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:60`.
    pub fn read_matrix_market_arma_line_60(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_arma (fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:60)")
    }

    /// Stub for `write_matrix_market_arma` at `fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:69`.
    pub fn write_matrix_market_arma_line_69(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_arma (fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:69)")
    }

    /// Stub for `write_matrix_market_arma` at `fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:82`.
    pub fn write_matrix_market_arma_line_82(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_arma (fast_matrix_market/include/fast_matrix_market/app/Armadillo.hpp:82)")
    }
}

pub mod app_array {
    use super::app_triplet::triplet_value_type;
    use super::chunking;
    use super::fast_matrix_market;
    use super::formatters;
    use super::header;
    use super::read_body;
    use super::types::{
        field_type, format_type, matrix_market_header, object_type, read_options, storage_order,
        symmetry_type, write_options,
    };
    use super::write_body;
    use super::Placeholder;
    use std::io::{BufRead, Write};

    /// Stub for `read_matrix_market_array` at `fast_matrix_market/include/fast_matrix_market/app/array.hpp:46`.
    pub fn read_matrix_market_array_line_46<V: triplet_value_type + Default>(
        instream: &mut impl BufRead,
        header: &mut matrix_market_header,
        order: storage_order,
        options: &read_options,
    ) -> Result<Vec<V>, super::fast_matrix_market::invalid_mm> {
        header::read_header_line_166(instream, header)?;
        V::check_header_field(header.field)?;
        if header.format == format_type::array
            && header.symmetry == symmetry_type::general
            && header.field != field_type::pattern
        {
            let mut values = vec![V::default(); (header.nrows * header.ncols).max(0) as usize];
            let mut row = 0i64;
            let mut col = 0i64;
            let mut file_line = header.header_line_count;
            let mut element_num = 0i64;
            loop {
                let chunk = chunking::get_next_chunk_line_51(instream, options)
                    .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
                if chunk.is_empty() {
                    break;
                }
                for raw_line in chunk.split_inclusive('\n') {
                    let bytes = raw_line.as_bytes();
                    let mut end = bytes.len();
                    while end > 0
                        && (bytes[end - 1] == b' '
                            || bytes[end - 1] == b'\t'
                            || bytes[end - 1] == b'\r'
                            || bytes[end - 1] == b'\n')
                    {
                        end -= 1;
                    }
                    let mut pos = 0usize;
                    while pos < end
                        && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r')
                    {
                        pos += 1;
                    }
                    if pos == end {
                        file_line += bytes.iter().filter(|&&c| c == b'\n').count() as i64;
                        continue;
                    }
                    if col >= header.ncols {
                        return Err(fast_matrix_market::invalid_mm_line_55(
                            "Too many values in array (file too long)".to_string(),
                            file_line + 1,
                        ));
                    }
                    let value_text = &raw_line[pos..end];
                    let mut fields = value_text.split_whitespace();
                    let value = if header.field == field_type::complex {
                        let real_field = fields.next().unwrap_or("");
                        let imaginary_field = fields.next().unwrap_or("");
                        read_body::read_real_or_complex_line_193::<V>(
                            &[real_field, imaginary_field],
                            header,
                            options,
                        )
                    } else {
                        let value_field = fields.next().unwrap_or("");
                        read_body::read_real_or_complex_line_193::<V>(
                            &[value_field],
                            header,
                            options,
                        )
                    }
                    .map_err(|err| {
                        fast_matrix_market::invalid_mm_line_55(err.msg, file_line + 1)
                    })?;
                    let offset = if order == storage_order::row_major {
                        row * header.ncols + col
                    } else {
                        col * header.nrows + row
                    };
                    values[offset as usize] = value;
                    row += 1;
                    if row == header.nrows {
                        row = 0;
                        col += 1;
                    }
                    file_line += 1;
                    element_num += 1;
                }
            }
            if element_num < header.nnz {
                return Err(fast_matrix_market::invalid_mm_line_54(format!(
                    "Truncated file. Expected another {} lines.",
                    header.nnz - element_num
                )));
            }
            return Ok(values);
        }
        let entries = read_body::read_matrix_market_body_line_562::<V>(instream, header, options)?;
        let mut values = vec![V::default(); (header.nrows * header.ncols).max(0) as usize];
        for (row, col, value) in entries {
            let offset = if order == storage_order::row_major {
                row * header.ncols + col
            } else {
                col * header.nrows + row
            };
            if let Some(slot) = values.get_mut(offset as usize) {
                *slot = value;
            }
        }
        Ok(values)
    }

    /// Stub for `read_matrix_market_array` at `fast_matrix_market/include/fast_matrix_market/app/array.hpp:66`.
    pub fn read_matrix_market_array_line_66<V: triplet_value_type + Default>(
        instream: &mut impl BufRead,
        order: storage_order,
        options: &read_options,
    ) -> Result<(i64, i64, Vec<V>), super::fast_matrix_market::invalid_mm> {
        let mut header = super::types::matrix_market_header_line_48();
        let values = read_matrix_market_array_line_46(instream, &mut header, order, options)?;
        Ok((header.nrows, header.ncols, values))
    }

    /// Stub for `read_matrix_market_array` at `fast_matrix_market/include/fast_matrix_market/app/array.hpp:82`.
    pub fn read_matrix_market_array_line_82<V: triplet_value_type + Default>(
        instream: &mut impl BufRead,
        order: storage_order,
        options: &read_options,
    ) -> Result<Vec<V>, super::fast_matrix_market::invalid_mm> {
        let mut header = super::types::matrix_market_header_line_48();
        read_matrix_market_array_line_46(instream, &mut header, order, options)
    }

    /// Stub for `write_matrix_market_array` at `fast_matrix_market/include/fast_matrix_market/app/array.hpp:94`.
    pub fn write_matrix_market_array_line_94<V: formatters::write_value_type>(
        os: &mut impl Write,
        mut header: matrix_market_header,
        values: Vec<V>,
        order: storage_order,
        options: &write_options,
    ) -> Result<(), String> {
        if header.nrows * header.ncols != values.len() as i64 {
            return Err("Array length does not match matrix dimensions.".to_string());
        }
        header.nnz = values.len() as i64;
        header.object = object_type::matrix;
        if options.fill_header_field_type {
            header.field = V::field_type();
        }
        header.format = format_type::array;
        header.symmetry = symmetry_type::general;
        header::write_header_line_278(os, &header, options.clone())
            .map_err(|err| err.to_string())?;
        let lf = formatters::line_formatter_line_20(header.clone(), options.clone());
        let mut formatter =
            formatters::array_formatter_line_314(lf, values, order, header.nrows, header.ncols);
        let mut chunks = Vec::new();
        while formatters::has_next_line_317(&formatter) {
            let chunk = formatters::next_chunk_line_351(&mut formatter, options);
            chunks.push(formatters::operator_line_326(&chunk));
        }
        write_body::write_body_line_66(os, &chunks, options).map_err(|err| err.to_string())
    }
}

pub mod app_blaze {
    use super::Placeholder;

    /// Original class `Blaze_CompressedMatrix_formatter` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:130`.
    #[derive(Clone, Debug, Default)]
    pub struct Blaze_CompressedMatrix_formatter {
        /// Original C++ type: `LF`.
        pub line_formatter: Placeholder,
        /// Original C++ type: `double`.
        pub nnz_per_major: f64,
        /// Original C++ type: `MatIndex`.
        pub major_iter: Placeholder,
        /// Original C++ type: `MatIndex`.
        pub major_size: Placeholder,
    }

    /// Original class `chunk` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:142`.
    #[derive(Clone, Debug, Default)]
    pub struct chunk {
        /// Original C++ type: `LF`.
        pub line_formatter: Placeholder,
        /// Original C++ type: `MatIndex`.
        pub major_iter: Placeholder,
        /// Original C++ type: `MatIndex`.
        pub major_end: Placeholder,
    }

    /// Stub for `read_matrix_market_blaze` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:17`.
    pub fn read_matrix_market_blaze_line_17(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_blaze (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:17)")
    }

    /// Stub for `read_matrix_market_blaze` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:97`.
    pub fn read_matrix_market_blaze_line_97(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_blaze (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:97)")
    }

    /// Stub for `read_matrix_market_blaze` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:117`.
    pub fn read_matrix_market_blaze_line_117(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_blaze (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:117)")
    }

    /// Stub for `Blaze_CompressedMatrix_formatter` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:134`.
    pub fn blaze_compressedmatrix_formatter_line_134(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: Blaze_CompressedMatrix_formatter (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:134)")
    }

    /// Stub for `has_next` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:138`.
    pub fn has_next_line_138() -> ! {
        panic!("untranslated fast_matrix_market function: has_next (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:138)")
    }

    /// Stub for `chunk` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:144`.
    pub fn chunk_line_144(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: chunk (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:144)")
    }

    /// Stub for `operator()` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:147`.
    pub fn operator_line_147() -> ! {
        panic!("untranslated fast_matrix_market function: operator() (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:147)")
    }

    /// Stub for `next_chunk` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:174`.
    pub fn next_chunk_line_174(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: next_chunk (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:174)")
    }

    /// Stub for `write_matrix_market_blaze` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:197`.
    pub fn write_matrix_market_blaze_line_197(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_blaze (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:197)")
    }

    /// Stub for `write_matrix_market_blaze` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:226`.
    pub fn write_matrix_market_blaze_line_226(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_blaze (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:226)")
    }

    /// Stub for `read_matrix_market_blaze` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:256`.
    pub fn read_matrix_market_blaze_line_256(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_blaze (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:256)")
    }

    /// Stub for `read_matrix_market_blaze` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:305`.
    pub fn read_matrix_market_blaze_line_305(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_blaze (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:305)")
    }

    /// Stub for `write_matrix_market_blaze` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:327`.
    pub fn write_matrix_market_blaze_line_327(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_blaze (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:327)")
    }

    /// Stub for `write_matrix_market_blaze` at `fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:372`.
    pub fn write_matrix_market_blaze_line_372(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_blaze (fast_matrix_market/include/fast_matrix_market/app/Blaze.hpp:372)")
    }
}

pub mod app_cxsparse {
    use super::Placeholder;

    /// Stub for `read_matrix_market_cxsparse` at `fast_matrix_market/include/fast_matrix_market/app/CXSparse.hpp:14`.
    pub fn read_matrix_market_cxsparse_line_14(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_cxsparse (fast_matrix_market/include/fast_matrix_market/app/CXSparse.hpp:14)")
    }

    /// Stub for `write_matrix_market_cxsparse` at `fast_matrix_market/include/fast_matrix_market/app/CXSparse.hpp:47`.
    pub fn write_matrix_market_cxsparse_line_47(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_cxsparse (fast_matrix_market/include/fast_matrix_market/app/CXSparse.hpp:47)")
    }
}

pub mod app_doublet {
    use super::app_triplet::triplet_value_type;
    use super::formatters;
    use super::header;
    use super::read_body;
    use super::types::{
        field_type, format_type, matrix_market_header, object_type, read_options, write_options,
    };
    use super::write_body;
    use super::Placeholder;
    use std::io::{BufRead, Write};

    /// Stub for `read_matrix_market_doublet` at `fast_matrix_market/include/fast_matrix_market/app/doublet.hpp:47`.
    pub fn read_matrix_market_doublet_line_47<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &mut matrix_market_header,
        options: &read_options,
    ) -> Result<(Vec<i64>, Vec<V>), super::fast_matrix_market::invalid_mm> {
        header::read_header_line_166(instream, header)?;
        let entries = read_body::read_matrix_market_body_line_562::<V>(instream, header, options)?;
        let mut indices = Vec::with_capacity(entries.len());
        let mut values = Vec::with_capacity(entries.len());
        for (row, col, value) in entries {
            let index = if header.ncols == 1 { row } else { col };
            indices.push(index);
            values.push(value);
        }
        Ok((indices, values))
    }

    /// Stub for `read_matrix_market_doublet` at `fast_matrix_market/include/fast_matrix_market/app/doublet.hpp:66`.
    pub fn read_matrix_market_doublet_line_66<V: triplet_value_type>(
        instream: &mut impl BufRead,
        options: &read_options,
    ) -> Result<(i64, Vec<i64>, Vec<V>), super::fast_matrix_market::invalid_mm> {
        let mut header = super::types::matrix_market_header_line_48();
        let (indices, values) =
            read_matrix_market_doublet_line_47::<V>(instream, &mut header, options)?;
        Ok((header.vector_length, indices, values))
    }

    /// Stub for `write_matrix_market_doublet` at `fast_matrix_market/include/fast_matrix_market/app/doublet.hpp:79`.
    pub fn write_matrix_market_doublet_line_79<V: formatters::write_value_type>(
        os: &mut impl Write,
        mut header: matrix_market_header,
        indices: Vec<i64>,
        values: Vec<V>,
        options: &write_options,
    ) -> Result<(), String> {
        header.nnz = indices.len() as i64;
        header.object = object_type::vector;
        if V::is_pattern() || (header.nnz > 0 && values.is_empty()) {
            header.field = field_type::pattern;
        } else if header.field != field_type::pattern && options.fill_header_field_type {
            header.field = V::field_type();
        }
        header.format = format_type::coordinate;
        header::write_header_line_278(os, &header, options.clone())
            .map_err(|err| err.to_string())?;
        let lf = formatters::line_formatter {
            header: header.clone(),
            options: options.clone(),
        };
        let mut formatter =
            formatters::triplet_formatter_line_120(lf, indices.clone(), indices, values)
                .map_err(|err| err.msg)?;
        let mut chunks = Vec::new();
        while formatters::has_next_line_134(&formatter) {
            let chunk = formatters::next_chunk_line_171(&mut formatter, options);
            let as_triplet = formatters::operator_line_149(&chunk);
            let mut vector_lines = String::new();
            for line in as_triplet.lines() {
                let mut parts = line.split_whitespace();
                if let Some(index) = parts.next() {
                    vector_lines.push_str(index);
                    let _duplicate_col = parts.next();
                    for part in parts {
                        vector_lines.push(' ');
                        vector_lines.push_str(part);
                    }
                    vector_lines.push('\n');
                }
            }
            chunks.push(vector_lines);
        }
        write_body::write_body_line_66(os, &chunks, options).map_err(|err| err.to_string())
    }
}

pub mod app_eigen {
    use super::Placeholder;

    /// Original class `sparse_Eigen_formatter` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:97`.
    #[derive(Clone, Debug, Default)]
    pub struct sparse_Eigen_formatter {
        /// Original C++ type: `LF`.
        pub line_formatter: Placeholder,
        /// Original C++ type: `double`.
        pub nnz_per_column: f64,
        /// Original C++ type: `MatIndex`.
        pub outer_iter: Placeholder,
    }

    /// Original class `chunk` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:108`.
    #[derive(Clone, Debug, Default)]
    pub struct chunk {
        /// Original C++ type: `LF`.
        pub line_formatter: Placeholder,
        /// Original C++ type: `MatIndex`.
        pub outer_iter: Placeholder,
        /// Original C++ type: `MatIndex`.
        pub outer_end: Placeholder,
    }

    /// Stub for `read_matrix_market_eigen` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:23`.
    pub fn read_matrix_market_eigen_line_23(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_eigen (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:23)")
    }

    /// Stub for `read_matrix_market_eigen` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:57`.
    pub fn read_matrix_market_eigen_line_57(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_eigen (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:57)")
    }

    /// Stub for `read_matrix_market_eigen_dense` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:69`.
    pub fn read_matrix_market_eigen_dense_line_69(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_eigen_dense (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:69)")
    }

    /// Stub for `read_matrix_market_eigen_dense` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:85`.
    pub fn read_matrix_market_eigen_dense_line_85(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_eigen_dense (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:85)")
    }

    /// Stub for `sparse_Eigen_formatter` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:100`.
    pub fn sparse_eigen_formatter_line_100(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: sparse_Eigen_formatter (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:100)")
    }

    /// Stub for `has_next` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:104`.
    pub fn has_next_line_104() -> ! {
        panic!("untranslated fast_matrix_market function: has_next (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:104)")
    }

    /// Stub for `chunk` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:110`.
    pub fn chunk_line_110(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: chunk (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:110)")
    }

    /// Stub for `operator()` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:113`.
    pub fn operator_line_113() -> ! {
        panic!("untranslated fast_matrix_market function: operator() (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:113)")
    }

    /// Stub for `next_chunk` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:132`.
    pub fn next_chunk_line_132(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: next_chunk (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:132)")
    }

    /// Stub for `write_matrix_market_eigen` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:154`.
    pub fn write_matrix_market_eigen_line_154(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_eigen (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:154)")
    }

    /// Stub for `write_matrix_market_eigen_dense` at `fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:178`.
    pub fn write_matrix_market_eigen_dense_line_178(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_eigen_dense (fast_matrix_market/include/fast_matrix_market/app/Eigen.hpp:178)")
    }
}

pub mod app_generator {
    use super::formatters;
    use super::header;
    use super::types::{field_type, format_type, matrix_market_header, object_type, write_options};
    use super::write_body;

    /// Original class `coo_independent_generator_formatter` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:15`.
    #[derive(Clone, Debug, Default)]
    pub struct coo_independent_generator_formatter<V = Vec<f64>> {
        /// Original C++ type: `LF`.
        pub line_formatter: Option<formatters::line_formatter>,
        /// Original C++ type: `int64_t`.
        pub nnz: i64,
        /// Original C++ type: `GEN_CALLABLE`.
        pub gen_callable: Option<fn(i64) -> (i64, i64, V)>,
        /// Original C++ type: `int64_t`.
        pub next_chunk_offset: i64,
    }

    /// Original class `chunk` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:28`.
    #[derive(Clone, Debug, Default)]
    pub struct chunk<V = Vec<f64>> {
        /// Original C++ type: `LF`.
        pub line_formatter: Option<formatters::line_formatter>,
        /// Original C++ type: `int64_t`.
        pub chunk_offset: i64,
        /// Original C++ type: `int64_t`.
        pub chunk_nnz: i64,
        /// Original C++ type: `GEN_CALLABLE`.
        pub gen_callable: Option<fn(i64) -> (i64, i64, V)>,
    }

    /// Stub for `coo_independent_generator_formatter` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:17`.
    pub fn coo_independent_generator_formatter_line_17<V: formatters::write_value_type>(
        lf: formatters::line_formatter,
        nnz: i64,
        gen_callable: fn(i64) -> (i64, i64, V),
    ) -> Result<coo_independent_generator_formatter<V>, super::fast_matrix_market::invalid_argument>
    {
        if nnz < 0 {
            return Err(super::fast_matrix_market::invalid_argument_line_77(
                "nnz cannot be negative.".to_string(),
            ));
        }
        Ok(coo_independent_generator_formatter {
            line_formatter: Some(lf),
            nnz,
            gen_callable: Some(gen_callable),
            next_chunk_offset: 0,
        })
    }

    /// Stub for `has_next` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:24`.
    pub fn has_next_line_24<V>(formatter: &coo_independent_generator_formatter<V>) -> bool {
        formatter.next_chunk_offset < formatter.nnz
    }

    /// Stub for `chunk` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:30`.
    pub fn chunk_line_30<V: formatters::write_value_type>(
        lf: formatters::line_formatter,
        chunk_offset: i64,
        chunk_nnz: i64,
        gen_callable: fn(i64) -> (i64, i64, V),
    ) -> chunk<V> {
        chunk {
            line_formatter: Some(lf),
            chunk_offset,
            chunk_nnz,
            gen_callable: Some(gen_callable),
        }
    }

    /// Stub for `operator()` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:34`.
    pub fn operator_line_34<V: formatters::write_value_type>(c: &chunk<V>) -> String {
        let mut out = String::with_capacity((c.chunk_nnz.max(0) as usize) * 25);
        if let (Some(lf), Some(gen_callable)) = (&c.line_formatter, c.gen_callable) {
            for i in 0..c.chunk_nnz {
                let (row, col, value) = gen_callable(c.chunk_offset + i);
                if V::is_pattern() {
                    out.push_str(&formatters::coord_matrix_pattern_line_42(row, col));
                } else {
                    out.push_str(&formatters::coord_matrix_line_23(
                        lf,
                        row,
                        col,
                        &value.as_fields(),
                    ));
                }
            }
        }
        out
    }

    /// Stub for `next_chunk` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:54`.
    pub fn next_chunk_line_54<V: formatters::write_value_type>(
        formatter: &mut coo_independent_generator_formatter<V>,
        options: &write_options,
    ) -> chunk<V> {
        let chunk_size = options
            .chunk_size_values
            .min(formatter.nnz - formatter.next_chunk_offset);
        let c = chunk {
            line_formatter: formatter.line_formatter.clone(),
            chunk_offset: formatter.next_chunk_offset,
            chunk_nnz: chunk_size,
            gen_callable: formatter.gen_callable,
        };
        formatter.next_chunk_offset += chunk_size;
        c
    }

    /// Stub for `write_matrix_market_generated_triplet` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:81`.
    pub fn write_matrix_market_generated_triplet_line_81<V: formatters::write_value_type>(
        os: &mut impl std::io::Write,
        mut header: matrix_market_header,
        nnz: i64,
        gen_callable: fn(i64) -> (i64, i64, V),
        options: &write_options,
    ) -> Result<(), String> {
        header.nnz = nnz;
        header.object = object_type::matrix;
        if V::is_pattern() {
            header.field = field_type::pattern;
        } else if header.field != field_type::pattern && options.fill_header_field_type {
            header.field = V::field_type();
        }
        header.format = format_type::coordinate;
        header::write_header_line_278(os, &header, options.clone())
            .map_err(|err| err.to_string())?;
        let lf = formatters::line_formatter_line_20(header, options.clone());
        let mut formatter = coo_independent_generator_formatter_line_17(lf, nnz, gen_callable)
            .map_err(|err| err.msg)?;
        let mut chunks = Vec::new();
        while has_next_line_24(&formatter) {
            let c = next_chunk_line_54(&mut formatter, options);
            chunks.push(operator_line_34(&c));
        }
        write_body::write_body_line_66(os, &chunks, options).map_err(|err| err.to_string())
    }
}

pub mod app_graphblas {
    use super::Placeholder;

    /// Original struct `GraphBLAS_typed` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:151`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed {}

    /// Original struct `GraphBLAS_typed<bool>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:179`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_bool_line_179 {}

    /// Original struct `GraphBLAS_typed<int8_t>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:207`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_int8_t_line_207 {}

    /// Original struct `GraphBLAS_typed<int16_t>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:235`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_int16_t_line_235 {}

    /// Original struct `GraphBLAS_typed<int32_t>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:263`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_int32_t_line_263 {}

    /// Original struct `GraphBLAS_typed<int64_t>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:291`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_int64_t_line_291 {}

    /// Original struct `GraphBLAS_typed<uint8_t>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:319`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_uint8_t_line_319 {}

    /// Original struct `GraphBLAS_typed<uint16_t>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:347`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_uint16_t_line_347 {}

    /// Original struct `GraphBLAS_typed<uint32_t>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:375`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_uint32_t_line_375 {}

    /// Original struct `GraphBLAS_typed<uint64_t>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:403`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_uint64_t_line_403 {}

    /// Original struct `GraphBLAS_typed<float>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:431`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_float_line_431 {}

    /// Original struct `GraphBLAS_typed<double>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:459`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_double_line_459 {}

    /// Original struct `GraphBLAS_typed<std::complex<float>>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:488`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_std_complex_float_line_488 {}

    /// Original struct `GraphBLAS_typed<std::complex<double>>` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:516`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GraphBLAS_typed_std_complex_double_line_516 {}

    /// Original struct `gblas_row_iter_impl` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:793`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct gblas_row_iter_impl {}

    /// Original struct `gblas_col_iter_impl` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:844`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct gblas_col_iter_impl {}

    /// Original struct `gblas_vec_iter_impl` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:896`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct gblas_vec_iter_impl {}

    /// Original class `GrB_Matrix_Iterator_formatter` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:953`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GrB_Matrix_Iterator_formatter {
        /// Original C++ type: `LF`.
        pub line_formatter: Placeholder,
        /// Original C++ type: `double`.
        pub nnz_per_kount: f64,
        /// Original C++ type: `GrB_Index`.
        pub kount_iter: Placeholder,
        /// Original C++ type: `GrB_Index`.
        pub kount: Placeholder,
    }

    /// Original class `chunk` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:966`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct chunk {
        /// Original C++ type: `LF`.
        pub line_formatter: Placeholder,
        /// Original C++ type: `GrB_Index`.
        pub kount_iter: Placeholder,
        /// Original C++ type: `GrB_Index`.
        pub kount_end: Placeholder,
    }

    /// Stub for `ok` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:69`.
    pub fn ok_line_69(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: ok (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:69)")
    }

    /// Stub for `parse_key` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:79`.
    pub fn parse_key_line_79(arg0: Placeholder, arg1: Placeholder, arg2: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: parse_key (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:79)")
    }

    /// Stub for `str_ctype_to_GrB_Type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:97`.
    pub fn str_ctype_to_grb_type_line_97() -> ! {
        panic!("untranslated fast_matrix_market function: str_ctype_to_GrB_Type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:97)")
    }

    /// Stub for `GrB_Type_to_header_type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:122`.
    pub fn grb_type_to_header_type_line_122() -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Type_to_header_type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:122)")
    }

    /// Stub for `parse_type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:139`.
    pub fn parse_type_line_139(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: parse_type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:139)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:152`.
    pub fn type_line_152() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:152)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:154`.
    pub fn build_matrix_line_154(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:154)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:158`.
    pub fn set_element_line_158(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:158)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:162`.
    pub fn grb_matrix_extracttuples_line_162(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:162)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:167`.
    pub fn gxb_iterator_get_line_167(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:167)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:171`.
    pub fn build_vector_line_171(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:171)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:175`.
    pub fn grb_vector_extracttuples_line_175(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:175)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:180`.
    pub fn type_line_180() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:180)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:182`.
    pub fn build_matrix_line_182(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:182)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:186`.
    pub fn set_element_line_186(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:186)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:190`.
    pub fn grb_matrix_extracttuples_line_190(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:190)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:195`.
    pub fn gxb_iterator_get_line_195(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:195)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:199`.
    pub fn build_vector_line_199(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:199)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:203`.
    pub fn grb_vector_extracttuples_line_203(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:203)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:208`.
    pub fn type_line_208() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:208)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:210`.
    pub fn build_matrix_line_210(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:210)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:214`.
    pub fn set_element_line_214(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:214)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:218`.
    pub fn grb_matrix_extracttuples_line_218(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:218)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:223`.
    pub fn gxb_iterator_get_line_223(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:223)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:227`.
    pub fn build_vector_line_227(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:227)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:231`.
    pub fn grb_vector_extracttuples_line_231(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:231)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:236`.
    pub fn type_line_236() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:236)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:238`.
    pub fn build_matrix_line_238(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:238)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:242`.
    pub fn set_element_line_242(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:242)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:246`.
    pub fn grb_matrix_extracttuples_line_246(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:246)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:251`.
    pub fn gxb_iterator_get_line_251(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:251)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:255`.
    pub fn build_vector_line_255(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:255)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:259`.
    pub fn grb_vector_extracttuples_line_259(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:259)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:264`.
    pub fn type_line_264() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:264)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:266`.
    pub fn build_matrix_line_266(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:266)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:270`.
    pub fn set_element_line_270(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:270)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:274`.
    pub fn grb_matrix_extracttuples_line_274(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:274)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:279`.
    pub fn gxb_iterator_get_line_279(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:279)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:283`.
    pub fn build_vector_line_283(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:283)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:287`.
    pub fn grb_vector_extracttuples_line_287(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:287)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:292`.
    pub fn type_line_292() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:292)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:294`.
    pub fn build_matrix_line_294(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:294)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:298`.
    pub fn set_element_line_298(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:298)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:302`.
    pub fn grb_matrix_extracttuples_line_302(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:302)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:307`.
    pub fn gxb_iterator_get_line_307(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:307)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:311`.
    pub fn build_vector_line_311(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:311)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:315`.
    pub fn grb_vector_extracttuples_line_315(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:315)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:320`.
    pub fn type_line_320() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:320)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:322`.
    pub fn build_matrix_line_322(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:322)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:326`.
    pub fn set_element_line_326(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:326)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:330`.
    pub fn grb_matrix_extracttuples_line_330(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:330)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:335`.
    pub fn gxb_iterator_get_line_335(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:335)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:339`.
    pub fn build_vector_line_339(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:339)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:343`.
    pub fn grb_vector_extracttuples_line_343(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:343)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:348`.
    pub fn type_line_348() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:348)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:350`.
    pub fn build_matrix_line_350(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:350)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:354`.
    pub fn set_element_line_354(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:354)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:358`.
    pub fn grb_matrix_extracttuples_line_358(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:358)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:363`.
    pub fn gxb_iterator_get_line_363(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:363)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:367`.
    pub fn build_vector_line_367(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:367)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:371`.
    pub fn grb_vector_extracttuples_line_371(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:371)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:376`.
    pub fn type_line_376() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:376)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:378`.
    pub fn build_matrix_line_378(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:378)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:382`.
    pub fn set_element_line_382(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:382)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:386`.
    pub fn grb_matrix_extracttuples_line_386(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:386)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:391`.
    pub fn gxb_iterator_get_line_391(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:391)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:395`.
    pub fn build_vector_line_395(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:395)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:399`.
    pub fn grb_vector_extracttuples_line_399(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:399)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:404`.
    pub fn type_line_404() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:404)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:406`.
    pub fn build_matrix_line_406(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:406)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:410`.
    pub fn set_element_line_410(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:410)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:414`.
    pub fn grb_matrix_extracttuples_line_414(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:414)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:419`.
    pub fn gxb_iterator_get_line_419(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:419)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:423`.
    pub fn build_vector_line_423(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:423)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:427`.
    pub fn grb_vector_extracttuples_line_427(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:427)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:432`.
    pub fn type_line_432() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:432)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:434`.
    pub fn build_matrix_line_434(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:434)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:438`.
    pub fn set_element_line_438(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:438)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:442`.
    pub fn grb_matrix_extracttuples_line_442(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:442)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:447`.
    pub fn gxb_iterator_get_line_447(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:447)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:451`.
    pub fn build_vector_line_451(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:451)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:455`.
    pub fn grb_vector_extracttuples_line_455(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:455)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:460`.
    pub fn type_line_460() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:460)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:462`.
    pub fn build_matrix_line_462(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:462)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:466`.
    pub fn set_element_line_466(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:466)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:470`.
    pub fn grb_matrix_extracttuples_line_470(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:470)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:475`.
    pub fn gxb_iterator_get_line_475(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:475)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:479`.
    pub fn build_vector_line_479(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:479)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:483`.
    pub fn grb_vector_extracttuples_line_483(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:483)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:489`.
    pub fn type_line_489() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:489)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:491`.
    pub fn build_matrix_line_491(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:491)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:495`.
    pub fn set_element_line_495(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:495)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:499`.
    pub fn grb_matrix_extracttuples_line_499(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:499)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:504`.
    pub fn gxb_iterator_get_line_504(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:504)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:508`.
    pub fn build_vector_line_508(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:508)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:512`.
    pub fn grb_vector_extracttuples_line_512(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:512)")
    }

    /// Stub for `type` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:517`.
    pub fn type_line_517() -> ! {
        panic!("untranslated fast_matrix_market function: type (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:517)")
    }

    /// Stub for `build_matrix` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:519`.
    pub fn build_matrix_line_519(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_matrix (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:519)")
    }

    /// Stub for `set_element` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:523`.
    pub fn set_element_line_523(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: set_element (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:523)")
    }

    /// Stub for `GrB_Matrix_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:527`.
    pub fn grb_matrix_extracttuples_line_527(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
        arg4: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:527)")
    }

    /// Stub for `GxB_Iterator_get` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:532`.
    pub fn gxb_iterator_get_line_532(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GxB_Iterator_get (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:532)")
    }

    /// Stub for `build_vector` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:536`.
    pub fn build_vector_line_536(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: build_vector (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:536)")
    }

    /// Stub for `GrB_Vector_extractTuples` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:540`.
    pub fn grb_vector_extracttuples_line_540(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Vector_extractTuples (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:540)")
    }

    /// Stub for `read_body_graphblas_coordinate` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:550`.
    pub fn read_body_graphblas_coordinate_line_550(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_body_graphblas_coordinate (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:550)")
    }

    /// Stub for `read_body_graphblas_array` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:602`.
    pub fn read_body_graphblas_array_line_602(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_body_graphblas_array (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:602)")
    }

    /// Stub for `read_body_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:623`.
    pub fn read_body_graphblas_line_623(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_body_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:623)")
    }

    /// Stub for `get_type_from_header` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:639`.
    pub fn get_type_from_header_line_639(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: get_type_from_header (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:639)")
    }

    /// Stub for `read_matrix_market_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:701`.
    pub fn read_matrix_market_graphblas_line_701(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:701)")
    }

    /// Stub for `read_matrix_market_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:758`.
    pub fn read_matrix_market_graphblas_line_758(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:758)")
    }

    /// Stub for `write_body_graphblas_triplet` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:771`.
    pub fn write_body_graphblas_triplet_line_771(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: write_body_graphblas_triplet (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:771)")
    }

    /// Stub for `setup` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:796`.
    pub fn setup_line_796(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: setup (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:796)")
    }

    /// Stub for `attach` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:809`.
    pub fn attach_line_809(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: attach (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:809)")
    }

    /// Stub for `kseek` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:819`.
    pub fn kseek_line_819(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: kseek (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:819)")
    }

    /// Stub for `getMajorIndex` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:823`.
    pub fn getmajorindex_line_823(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: getMajorIndex (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:823)")
    }

    /// Stub for `getMinorIndex` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:827`.
    pub fn getminorindex_line_827(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: getMinorIndex (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:827)")
    }

    /// Stub for `nextMinor` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:831`.
    pub fn nextminor_line_831(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: nextMinor (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:831)")
    }

    /// Stub for `nextMajor` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:835`.
    pub fn nextmajor_line_835(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: nextMajor (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:835)")
    }

    /// Stub for `to_row_col` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:839`.
    pub fn to_row_col_line_839(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: to_row_col (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:839)")
    }

    /// Stub for `setup` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:847`.
    pub fn setup_line_847(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: setup (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:847)")
    }

    /// Stub for `attach` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:860`.
    pub fn attach_line_860(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: attach (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:860)")
    }

    /// Stub for `kseek` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:870`.
    pub fn kseek_line_870(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: kseek (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:870)")
    }

    /// Stub for `getMinorIndex` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:874`.
    pub fn getminorindex_line_874(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: getMinorIndex (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:874)")
    }

    /// Stub for `getMajorIndex` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:878`.
    pub fn getmajorindex_line_878(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: getMajorIndex (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:878)")
    }

    /// Stub for `nextMinor` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:882`.
    pub fn nextminor_line_882(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: nextMinor (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:882)")
    }

    /// Stub for `nextMajor` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:886`.
    pub fn nextmajor_line_886(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: nextMajor (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:886)")
    }

    /// Stub for `to_row_col` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:890`.
    pub fn to_row_col_line_890(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: to_row_col (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:890)")
    }

    /// Stub for `setup` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:899`.
    pub fn setup_line_899(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: setup (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:899)")
    }

    /// Stub for `attach` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:913`.
    pub fn attach_line_913(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: attach (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:913)")
    }

    /// Stub for `kseek` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:923`.
    pub fn kseek_line_923(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: kseek (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:923)")
    }

    /// Stub for `getMinorIndex` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:927`.
    pub fn getminorindex_line_927(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: getMinorIndex (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:927)")
    }

    /// Stub for `getMajorIndex` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:931`.
    pub fn getmajorindex_line_931(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: getMajorIndex (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:931)")
    }

    /// Stub for `nextMinor` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:935`.
    pub fn nextminor_line_935(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: nextMinor (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:935)")
    }

    /// Stub for `nextMajor` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:939`.
    pub fn nextmajor_line_939(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: nextMajor (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:939)")
    }

    /// Stub for `to_row_col` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:943`.
    pub fn to_row_col_line_943(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: to_row_col (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:943)")
    }

    /// Stub for `GrB_Matrix_Iterator_formatter` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:955`.
    pub fn grb_matrix_iterator_formatter_line_955(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: GrB_Matrix_Iterator_formatter (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:955)")
    }

    /// Stub for `has_next` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:962`.
    pub fn has_next_line_962() -> ! {
        panic!("untranslated fast_matrix_market function: has_next (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:962)")
    }

    /// Stub for `chunk` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:968`.
    pub fn chunk_line_968(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: chunk (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:968)")
    }

    /// Stub for `operator()` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:971`.
    pub fn operator_line_971() -> ! {
        panic!("untranslated fast_matrix_market function: operator() (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:971)")
    }

    /// Stub for `next_chunk` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1007`.
    pub fn next_chunk_line_1007(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: next_chunk (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1007)")
    }

    /// Stub for `write_body_graphblas_array_iterator` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1033`.
    pub fn write_body_graphblas_array_iterator_line_1033(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: write_body_graphblas_array_iterator (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1033)")
    }

    /// Stub for `write_body_graphblas_array_row_via_extract` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1053`.
    pub fn write_body_graphblas_array_row_via_extract_line_1053(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: write_body_graphblas_array_row_via_extract (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1053)")
    }

    /// Stub for `write_body_graphblas_array_via_triplet` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1073`.
    pub fn write_body_graphblas_array_via_triplet_line_1073(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: write_body_graphblas_array_via_triplet (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1073)")
    }

    /// Stub for `write_body_graphblas_iterator` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1115`.
    pub fn write_body_graphblas_iterator_line_1115(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: write_body_graphblas_iterator (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1115)")
    }

    /// Stub for `write_body_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1141`.
    pub fn write_body_graphblas_line_1141(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: write_body_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1141)")
    }

    /// Stub for `get_field` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1175`.
    pub fn get_field_line_1175(arg0: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: get_field (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1175)")
    }

    /// Stub for `add_structured_comment` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1194`.
    pub fn add_structured_comment_line_1194(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: add_structured_comment (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1194)")
    }

    /// Stub for `write_matrix_market_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1220`.
    pub fn write_matrix_market_graphblas_line_1220(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1220)")
    }

    /// Stub for `read_body_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1305`.
    pub fn read_body_graphblas_line_1305(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_body_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1305)")
    }

    /// Stub for `read_matrix_market_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1324`.
    pub fn read_matrix_market_graphblas_line_1324(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1324)")
    }

    /// Stub for `read_matrix_market_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1382`.
    pub fn read_matrix_market_graphblas_line_1382(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: read_matrix_market_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1382)")
    }

    /// Stub for `write_body_graphblas_doublet` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1395`.
    pub fn write_body_graphblas_doublet_line_1395(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: write_body_graphblas_doublet (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1395)")
    }

    /// Stub for `write_body_graphblas_iterator` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1420`.
    pub fn write_body_graphblas_iterator_line_1420(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: write_body_graphblas_iterator (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1420)")
    }

    /// Stub for `write_body_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1434`.
    pub fn write_body_graphblas_line_1434(
        arg0: Placeholder,
        arg1: Placeholder,
        arg2: Placeholder,
        arg3: Placeholder,
    ) -> ! {
        panic!("untranslated fast_matrix_market function: write_body_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1434)")
    }

    /// Stub for `write_matrix_market_graphblas` at `fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1448`.
    pub fn write_matrix_market_graphblas_line_1448(arg0: Placeholder, arg1: Placeholder) -> ! {
        panic!("untranslated fast_matrix_market function: write_matrix_market_graphblas (fast_matrix_market/include/fast_matrix_market/app/GraphBLAS.hpp:1448)")
    }
}

pub mod app_triplet {
    use super::chunking;
    use super::fast_matrix_market;
    use super::field_conv;
    use super::formatters;
    use super::header;
    use super::read_body::{self, line_counts};
    use super::types::{
        field_type, format_type, matrix_market_header, object_type, read_options, symmetry_type,
        write_options,
    };
    use super::write_body;
    use super::Placeholder;
    use std::io::{BufRead, Write};

    pub trait triplet_value_type: Clone {
        fn check_header_field(
            field: field_type,
        ) -> Result<(), super::fast_matrix_market::invalid_mm> {
            if field == field_type::complex && !Self::can_read_complex() {
                return Err(fast_matrix_market::invalid_mm_line_54(
                    "Matrix Market file has complex fields but passed data structure cannot handle complex values."
                        .to_string(),
                ));
            }
            Ok(())
        }

        fn can_read_complex() -> bool {
            false
        }

        fn pattern_value() -> Self;
        fn real_value(value: f64) -> Self;
        fn complex_value(
            real: f64,
            imaginary: f64,
        ) -> Result<Self, super::fast_matrix_market::invalid_mm>;
        fn symmetric_value(&self, symmetry: symmetry_type) -> Self;
        fn zero_value_like(&self) -> Self;
    }

    impl triplet_value_type for f64 {
        fn pattern_value() -> Self {
            1.0
        }

        fn real_value(value: f64) -> Self {
            value
        }

        fn complex_value(
            _real: f64,
            _imaginary: f64,
        ) -> Result<Self, super::fast_matrix_market::invalid_mm> {
            Err(fast_matrix_market::invalid_mm_line_54(
                "Matrix Market file has complex fields but passed data structure cannot handle complex values."
                    .to_string(),
            ))
        }

        fn symmetric_value(&self, symmetry: symmetry_type) -> Self {
            match symmetry {
                symmetry_type::skew_symmetric => -*self,
                _ => *self,
            }
        }

        fn zero_value_like(&self) -> Self {
            0.0
        }
    }

    impl triplet_value_type for (f64, f64) {
        fn can_read_complex() -> bool {
            true
        }

        fn pattern_value() -> Self {
            (1.0, 0.0)
        }

        fn real_value(value: f64) -> Self {
            (value, 0.0)
        }

        fn complex_value(
            real: f64,
            imaginary: f64,
        ) -> Result<Self, super::fast_matrix_market::invalid_mm> {
            Ok((real, imaginary))
        }

        fn symmetric_value(&self, symmetry: symmetry_type) -> Self {
            match symmetry {
                symmetry_type::skew_symmetric => (-self.0, -self.1),
                symmetry_type::hermitian => (self.0, -self.1),
                _ => *self,
            }
        }

        fn zero_value_like(&self) -> Self {
            (0.0, 0.0)
        }
    }

    impl triplet_value_type for () {
        fn check_header_field(
            _field: field_type,
        ) -> Result<(), super::fast_matrix_market::invalid_mm> {
            Ok(())
        }

        fn pattern_value() -> Self {}

        fn real_value(_value: f64) -> Self {}

        fn complex_value(
            _real: f64,
            _imaginary: f64,
        ) -> Result<Self, super::fast_matrix_market::invalid_mm> {
            Ok(())
        }

        fn symmetric_value(&self, _symmetry: symmetry_type) -> Self {}

        fn zero_value_like(&self) -> Self {}
    }

    impl triplet_value_type for Vec<f64> {
        fn can_read_complex() -> bool {
            true
        }

        fn pattern_value() -> Self {
            Vec::new()
        }

        fn real_value(value: f64) -> Self {
            vec![value]
        }

        fn complex_value(
            real: f64,
            imaginary: f64,
        ) -> Result<Self, super::fast_matrix_market::invalid_mm> {
            Ok(vec![real, imaginary])
        }

        fn symmetric_value(&self, symmetry: symmetry_type) -> Self {
            match symmetry {
                symmetry_type::skew_symmetric => self.iter().map(|value| -*value).collect(),
                symmetry_type::hermitian if self.len() == 2 => vec![self[0], -self[1]],
                _ => self.clone(),
            }
        }

        fn zero_value_like(&self) -> Self {
            vec![0.0; self.len()]
        }
    }

    /// Stub for `generalize_symmetry_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:45`.
    pub fn generalize_symmetry_triplet_line_45<V: triplet_value_type>(
        rows: &mut Vec<i64>,
        cols: &mut Vec<i64>,
        values: &mut Vec<V>,
        symmetry: symmetry_type,
    ) {
        if symmetry == symmetry_type::general {
            return;
        }
        let orig_size = rows.len();
        for i in 0..orig_size {
            if rows[i] == cols[i] {
                continue;
            }
            rows.push(cols[i]);
            cols.push(rows[i]);
            values.push(values[i].symmetric_value(symmetry));
        }
    }

    /// Stub for `read_matrix_market_body_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:84`.
    pub fn read_matrix_market_body_triplet_line_84<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<(Vec<i64>, Vec<i64>, Vec<V>), super::fast_matrix_market::invalid_mm> {
        V::check_header_field(header.field)?;
        let mut app_options = options.clone();
        let app_generalize = app_options.generalize_symmetry && app_options.generalize_symmetry_app;
        if app_generalize {
            app_options.generalize_symmetry = false;
        }
        let base_nnz = header.nnz.max(0) as usize;
        let mut rows = Vec::with_capacity(base_nnz);
        let mut cols = Vec::with_capacity(base_nnz);
        let mut values = Vec::<V>::with_capacity(base_nnz);
        if header.format == format_type::coordinate
            && (header.symmetry == symmetry_type::general || !app_options.generalize_symmetry)
        {
            let mut lc = line_counts {
                file_line: header.header_line_count,
                element_num: 0,
            };
            loop {
                let chunk = chunking::get_next_chunk_line_51(instream, &app_options)
                    .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
                if chunk.is_empty() {
                    break;
                }
                for raw_line in chunk.split_inclusive('\n') {
                    let bytes = raw_line.as_bytes();
                    let mut end = bytes.len();
                    while end > 0
                        && (bytes[end - 1] == b' '
                            || bytes[end - 1] == b'\t'
                            || bytes[end - 1] == b'\r'
                            || bytes[end - 1] == b'\n')
                    {
                        end -= 1;
                    }
                    let mut pos = 0usize;
                    while pos < end
                        && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r')
                    {
                        pos += 1;
                    }
                    if pos == end {
                        lc.file_line += bytes.iter().filter(|&&c| c == b'\n').count() as i64;
                        continue;
                    }
                    if lc.element_num >= header.nnz {
                        return Err(fast_matrix_market::invalid_mm_line_55(
                            "Too many lines in file (file too long)".to_string(),
                            lc.file_line + 1,
                        ));
                    }
                    let row_start = pos;
                    while pos < end
                        && bytes[pos] != b' '
                        && bytes[pos] != b'\t'
                        && bytes[pos] != b'\r'
                        && bytes[pos] != b'\n'
                    {
                        pos += 1;
                    }
                    let row_field = &raw_line[row_start..pos];
                    let (row_end, row_one) =
                        field_conv::read_int_line_140(row_field, 0).map_err(|_| {
                            fast_matrix_market::invalid_mm_line_55(
                                "Invalid integer value.".to_string(),
                                lc.file_line + 1,
                            )
                        })?;
                    if row_end != row_field.len() {
                        return Err(fast_matrix_market::invalid_mm_line_55(
                            "Invalid integer value.".to_string(),
                            lc.file_line + 1,
                        ));
                    }
                    let (row_zero, col_zero) = if header.object == object_type::matrix {
                        while pos < end
                            && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r')
                        {
                            pos += 1;
                        }
                        let col_start = pos;
                        while pos < end
                            && bytes[pos] != b' '
                            && bytes[pos] != b'\t'
                            && bytes[pos] != b'\r'
                            && bytes[pos] != b'\n'
                        {
                            pos += 1;
                        }
                        let col_field = &raw_line[col_start..pos];
                        let (col_end, col_one) = field_conv::read_int_line_140(col_field, 0)
                            .map_err(|_| {
                                fast_matrix_market::invalid_mm_line_55(
                                    "Invalid integer value.".to_string(),
                                    lc.file_line + 1,
                                )
                            })?;
                        if col_end != col_field.len() {
                            return Err(fast_matrix_market::invalid_mm_line_55(
                                "Invalid integer value.".to_string(),
                                lc.file_line + 1,
                            ));
                        }
                        if row_one <= 0 || row_one > header.nrows {
                            return Err(fast_matrix_market::invalid_mm_line_55(
                                "Row index out of bounds".to_string(),
                                lc.file_line + 1,
                            ));
                        }
                        if col_one <= 0 || col_one > header.ncols {
                            return Err(fast_matrix_market::invalid_mm_line_55(
                                "Column index out of bounds".to_string(),
                                lc.file_line + 1,
                            ));
                        }
                        (row_one - 1, col_one - 1)
                    } else {
                        if row_one <= 0 || row_one > header.vector_length {
                            return Err(fast_matrix_market::invalid_mm_line_55(
                                "Vector index out of bounds".to_string(),
                                lc.file_line + 1,
                            ));
                        }
                        (row_one - 1, 0)
                    };
                    rows.push(row_zero);
                    cols.push(col_zero);
                    if header.field != field_type::pattern {
                        while pos < end
                            && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r')
                        {
                            pos += 1;
                        }
                        let value_start = pos;
                        while pos < end
                            && bytes[pos] != b' '
                            && bytes[pos] != b'\t'
                            && bytes[pos] != b'\r'
                            && bytes[pos] != b'\n'
                        {
                            pos += 1;
                        }
                        if value_start == pos {
                            return Err(fast_matrix_market::invalid_mm_line_55(
                                "Invalid floating-point value.".to_string(),
                                lc.file_line + 1,
                            ));
                        }
                        if header.field == field_type::complex {
                            let real_field = &raw_line[value_start..pos];
                            while pos < end
                                && (bytes[pos] == b' '
                                    || bytes[pos] == b'\t'
                                    || bytes[pos] == b'\r')
                            {
                                pos += 1;
                            }
                            let imag_start = pos;
                            while pos < end
                                && bytes[pos] != b' '
                                && bytes[pos] != b'\t'
                                && bytes[pos] != b'\r'
                                && bytes[pos] != b'\n'
                            {
                                pos += 1;
                            }
                            if imag_start == pos {
                                return Err(fast_matrix_market::invalid_mm_line_55(
                                    "Invalid floating-point value.".to_string(),
                                    lc.file_line + 1,
                                ));
                            }
                            let imaginary_field = &raw_line[imag_start..pos];
                            let value = read_body::read_real_or_complex_line_193::<V>(
                                &[real_field, imaginary_field],
                                header,
                                &app_options,
                            )
                            .map_err(|err| {
                                fast_matrix_market::invalid_mm_line_55(err.msg, lc.file_line + 1)
                            })?;
                            values.push(value);
                        } else {
                            let value_field = &raw_line[value_start..pos];
                            let value = read_body::read_real_or_complex_line_193::<V>(
                                &[value_field],
                                header,
                                &app_options,
                            )
                            .map_err(|err| {
                                fast_matrix_market::invalid_mm_line_55(err.msg, lc.file_line + 1)
                            })?;
                            values.push(value);
                        };
                    } else {
                        values.push(V::pattern_value());
                    }
                    lc.file_line += 1;
                    lc.element_num += 1;
                }
            }
            if lc.element_num < header.nnz {
                return Err(fast_matrix_market::invalid_mm_line_54(format!(
                    "Truncated file. Expected another {} lines.",
                    header.nnz - lc.element_num
                )));
            }
            if app_generalize && header.symmetry != symmetry_type::general {
                let orig_size = rows.len();
                for i in 0..orig_size {
                    if rows[i] == cols[i] {
                        continue;
                    }
                    rows.push(cols[i]);
                    cols.push(rows[i]);
                    let value = values[i].symmetric_value(header.symmetry);
                    values.push(value);
                }
            }
            return Ok((rows, cols, values));
        }
        let mut lc = line_counts {
            file_line: header.header_line_count,
            element_num: 0,
        };
        let mut row = 0i64;
        let mut col = 0i64;
        loop {
            let chunk = chunking::get_next_chunk_line_51(instream, &app_options)
                .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
            if chunk.is_empty() {
                break;
            }
            let (next_lc, next_entries) = if header.format == format_type::coordinate {
                if header.object == object_type::matrix {
                    read_body::read_chunk_matrix_coordinate_line_213::<V>(
                        &chunk,
                        header,
                        lc,
                        &app_options,
                    )?
                } else {
                    read_body::read_chunk_vector_coordinate_line_281::<V>(
                        &chunk,
                        header,
                        lc,
                        &app_options,
                    )?
                }
            } else {
                read_body::read_chunk_array_line_332::<V>(
                    &chunk,
                    header,
                    lc,
                    &app_options,
                    &mut row,
                    &mut col,
                )?
            };
            lc = next_lc;
            rows.reserve(next_entries.len());
            cols.reserve(next_entries.len());
            for (entry_row, entry_col, value) in next_entries {
                rows.push(entry_row);
                cols.push(entry_col);
                values.push(value);
            }
        }
        if lc.element_num < header.nnz
            && !(header.symmetry != symmetry_type::general && header.format == format_type::array)
        {
            return Err(fast_matrix_market::invalid_mm_line_54(format!(
                "Truncated file. Expected another {} lines.",
                header.nnz - lc.element_num
            )));
        }
        if app_generalize && header.symmetry != symmetry_type::general {
            let orig_size = rows.len();
            for i in 0..orig_size {
                if rows[i] == cols[i] {
                    continue;
                }
                rows.push(cols[i]);
                cols.push(rows[i]);
                let value = values[i].symmetric_value(header.symmetry);
                values.push(value);
            }
        }
        Ok((rows, cols, values))
    }

    /// Stub for `read_matrix_market_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:112`.
    pub fn read_matrix_market_triplet_line_112<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &mut matrix_market_header,
        options: &read_options,
    ) -> Result<(Vec<i64>, Vec<i64>, Vec<V>), super::fast_matrix_market::invalid_mm> {
        header::read_header_line_166(instream, header)?;
        read_matrix_market_body_triplet_line_84::<V>(instream, header, options)
    }

    /// Stub for `read_matrix_market_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:126`.
    pub fn read_matrix_market_triplet_line_126<V: triplet_value_type>(
        instream: &mut impl BufRead,
        options: &read_options,
    ) -> Result<(i64, i64, Vec<i64>, Vec<i64>, Vec<V>), super::fast_matrix_market::invalid_mm> {
        let mut header = super::types::matrix_market_header_line_48();
        let (rows, cols, values) =
            read_matrix_market_triplet_line_112::<V>(instream, &mut header, options)?;
        Ok((header.nrows, header.ncols, rows, cols, values))
    }

    /// Stub for `write_matrix_market_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:140`.
    pub fn write_matrix_market_triplet_line_140<V: formatters::write_value_type>(
        os: &mut impl Write,
        mut header: matrix_market_header,
        rows: Vec<i64>,
        cols: Vec<i64>,
        values: Vec<V>,
        options: &write_options,
    ) -> Result<(), String> {
        header.nnz = rows.len() as i64;
        header.object = object_type::matrix;
        if V::is_pattern() || (header.nnz > 0 && values.is_empty()) {
            header.field = field_type::pattern;
        } else if header.field != field_type::pattern && options.fill_header_field_type {
            header.field = V::field_type();
        }
        header.format = format_type::coordinate;
        header::write_header_line_278(os, &header, options.clone())
            .map_err(|err| err.to_string())?;
        let lf = formatters::line_formatter_line_20(header, options.clone());
        let mut formatter = formatters::triplet_formatter_line_120(lf, rows, cols, values)
            .map_err(|err| err.msg)?;
        let mut chunks = Vec::new();
        while formatters::has_next_line_134(&formatter) {
            let chunk = formatters::next_chunk_line_171(&mut formatter, options);
            chunks.push(formatters::operator_line_149(&chunk));
        }
        write_body::write_body_line_66(os, &chunks, options).map_err(|err| err.to_string())
    }

    /// Stub for `write_matrix_market_csc` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:173`.
    pub fn write_matrix_market_csc_line_173<V: formatters::write_value_type>(
        os: &mut impl Write,
        mut header: matrix_market_header,
        indptr: Vec<usize>,
        indices: Vec<i64>,
        values: Vec<V>,
        is_csr: bool,
        options: &write_options,
    ) -> Result<(), String> {
        header.nnz = indices.len() as i64;
        header.object = object_type::matrix;
        if V::is_pattern() || (header.nnz > 0 && values.is_empty()) {
            header.field = field_type::pattern;
        } else if header.field != field_type::pattern && options.fill_header_field_type {
            header.field = V::field_type();
        }
        header.format = format_type::coordinate;
        header::write_header_line_278(os, &header, options.clone())
            .map_err(|err| err.to_string())?;
        let lf = formatters::line_formatter_line_20(header, options.clone());
        let mut formatter = formatters::csc_formatter_line_202(lf, indptr, indices, values, is_csr)
            .map_err(|err| err.msg)?;
        let mut chunks = Vec::new();
        while formatters::has_next_line_221(&formatter) {
            let chunk = formatters::next_chunk_line_282(&mut formatter, options);
            chunks.push(formatters::operator_line_238(&chunk));
        }
        write_body::write_body_line_66(os, &chunks, options).map_err(|err| err.to_string())
    }
}

pub mod app_user_type_string {
    use super::types::field_type;
    use super::Placeholder;

    /// Original struct `can_read_complex<std::string>` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:52`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct can_read_complex_std_string_line_52 {}

    /// Stub for `read_value` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:38`.
    pub fn read_value_line_38(s: &str, mut pos: usize) -> (usize, String) {
        let field_start = pos;
        let bytes = s.as_bytes();
        while pos < bytes.len() && bytes[pos] != b'\n' {
            pos += 1;
        }
        (pos, s[field_start..pos].to_string())
    }

    /// Stub for `negate` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:57`.
    pub fn negate_line_57(o: &str) -> String {
        format!("-{o}")
    }

    /// Stub for `pattern_default_value` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:64`.
    pub fn pattern_default_value_line_64() -> String {
        String::new()
    }

    /// Stub for `get_field_type` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:76`.
    pub fn get_field_type_line_76() -> field_type {
        field_type::real
    }

    /// Stub for `value_to_string` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:83`.
    pub fn value_to_string_line_83(value: &str, _precision: i32) -> String {
        value.to_string()
    }
}

pub mod chunking {
    use super::types::read_options;
    use super::Placeholder;
    use std::io::{BufRead, Read};

    /// Stub for `get_next_chunk` at `fast_matrix_market/include/fast_matrix_market/chunking.hpp:11`.
    pub fn get_next_chunk_line_11<R: BufRead>(
        chunk: &mut String,
        instream: &mut R,
        options: &read_options,
    ) -> std::io::Result<()> {
        const CHUNK_EXTRA: usize = 4096;
        chunk.clear();
        let chunk_size = options.chunk_size_bytes.max(0) as usize;
        let bytes_to_read = chunk_size.saturating_sub(CHUNK_EXTRA);

        if bytes_to_read > 0 {
            let mut buf = vec![0u8; bytes_to_read];
            let mut num_read = 0usize;
            while num_read < bytes_to_read {
                let n = instream.read(&mut buf[num_read..])?;
                if n == 0 {
                    break;
                }
                num_read += n;
            }
            chunk.push_str(&String::from_utf8_lossy(&buf[..num_read]));
            if num_read == 0 || num_read < bytes_to_read || chunk.ends_with('\n') {
                return Ok(());
            }
        }

        let mut suffix = String::new();
        let bytes = instream.read_line(&mut suffix)?;
        if bytes > 0 && !suffix.ends_with('\n') {
            chunk.push_str(&suffix);
        } else {
            chunk.push_str(&suffix);
        }
        Ok(())
    }

    /// Stub for `get_next_chunk` at `fast_matrix_market/include/fast_matrix_market/chunking.hpp:51`.
    pub fn get_next_chunk_line_51<R: BufRead>(
        instream: &mut R,
        options: &read_options,
    ) -> std::io::Result<String> {
        let mut chunk = String::new();
        get_next_chunk_line_11(&mut chunk, instream, options)?;
        Ok(chunk)
    }

    /// Stub for `is_all_spaces` at `fast_matrix_market/include/fast_matrix_market/chunking.hpp:59`.
    pub fn is_all_spaces_line_59(text: &str) -> bool {
        text.bytes().all(|c| c == b' ' || c == b'\t' || c == b'\r')
    }

    /// Stub for `count_lines` at `fast_matrix_market/include/fast_matrix_market/chunking.hpp:66`.
    pub fn count_lines_line_66(chunk: &str) -> (i64, i64) {
        let mut num_newlines = 0i64;
        let mut num_empty_lines = 0i64;
        let mut line_start = 0usize;

        for (pos, c) in chunk.char_indices() {
            if c == '\n' {
                num_newlines += 1;
                if is_all_spaces_line_59(&chunk[line_start..pos]) {
                    num_empty_lines += 1;
                }
                line_start = pos + 1;
            }
        }

        if line_start != chunk.len() && is_all_spaces_line_59(&chunk[line_start..]) {
            num_empty_lines += 1;
        }

        if num_newlines == 0 {
            if chunk.is_empty() {
                num_empty_lines = 1;
            }
            return (1, num_empty_lines);
        }

        if !chunk.ends_with('\n') {
            num_newlines += 1;
        }

        (num_newlines, num_empty_lines)
    }
}

pub mod fast_matrix_market {
    use super::Placeholder;

    pub const FAST_MATRIX_MARKET_VERSION_MAJOR: i64 = 1;
    pub const FAST_MATRIX_MARKET_VERSION_MINOR: i64 = 7;
    pub const FAST_MATRIX_MARKET_VERSION_PATCH: i64 = 7;
    pub const K_SPACE: &str = " ";
    pub const K_NEWLINE: &str = "\n";

    /// Original class `fmm_error` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:38`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct fmm_error {
        /// Original C++ type: `std::string`.
        pub msg: String,
    }

    /// Original class `invalid_mm` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:52`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct invalid_mm {
        pub msg: String,
    }

    /// Original class `out_of_range` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:67`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct out_of_range {
        pub msg: String,
    }

    /// Original class `invalid_argument` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:75`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct invalid_argument {
        pub msg: String,
    }

    /// Original class `complex_incompatible` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:83`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct complex_incompatible {
        pub msg: String,
    }

    /// Original class `support_not_selected` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:91`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct support_not_selected {
        pub msg: String,
    }

    /// Original class `no_vector_support` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:99`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct no_vector_support {
        pub msg: String,
    }

    /// Original struct `pattern_placeholder_type` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:108`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct pattern_placeholder_type {}

    /// Stub for `fmm_error` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:40`.
    pub fn fmm_error_line_40(msg: String) -> fmm_error {
        fmm_error { msg }
    }

    /// Stub for `what` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:42`.
    pub fn what_line_42(err: &fmm_error) -> &str {
        err.msg.as_str()
    }

    /// Stub for `invalid_mm` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:54`.
    pub fn invalid_mm_line_54(msg: String) -> invalid_mm {
        invalid_mm { msg }
    }

    /// Stub for `invalid_mm` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:55`.
    pub fn invalid_mm_line_55(msg: String, line_num: i64) -> invalid_mm {
        let mut err = invalid_mm { msg };
        prepend_line_number_line_59(&mut err, line_num);
        err
    }

    /// Stub for `prepend_line_number` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:59`.
    pub fn prepend_line_number_line_59(err: &mut invalid_mm, line_num: i64) {
        err.msg = format!("Line {line_num}: {}", err.msg);
    }

    /// Stub for `out_of_range` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:69`.
    pub fn out_of_range_line_69(msg: String) -> out_of_range {
        out_of_range { msg }
    }

    /// Stub for `invalid_argument` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:77`.
    pub fn invalid_argument_line_77(msg: String) -> invalid_argument {
        invalid_argument { msg }
    }

    /// Stub for `complex_incompatible` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:85`.
    pub fn complex_incompatible_line_85(msg: String) -> complex_incompatible {
        complex_incompatible { msg }
    }

    /// Stub for `support_not_selected` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:93`.
    pub fn support_not_selected_line_93(msg: String) -> support_not_selected {
        support_not_selected { msg }
    }

    /// Stub for `no_vector_support` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:101`.
    pub fn no_vector_support_line_101(msg: String) -> no_vector_support {
        no_vector_support { msg }
    }

    /// Stub for `operator-` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:114`.
    pub fn operator_line_114(o: pattern_placeholder_type) -> pattern_placeholder_type {
        o
    }

    /// Stub for `negate` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:119`.
    pub fn negate_line_119(o: bool) -> bool {
        !o
    }

    /// Stub for `negate` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:123`.
    pub fn negate_line_123(o: bool) -> bool {
        !o
    }

    /// Stub for `negate` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:128`.
    pub fn negate_line_128<T>(o: T) -> T
    where
        T: std::ops::Neg<Output = T>,
    {
        -o
    }

    /// Stub for `pattern_default_value` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:133`.
    pub fn pattern_default_value_line_133<T>() -> T
    where
        T: From<u8>,
    {
        1u8.into()
    }

    /// Stub for `get_zero` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:141`.
    pub fn get_zero_line_141<T>() -> T
    where
        T: Default,
    {
        T::default()
    }

    /// Stub for `is_ready` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:150`.
    pub fn is_ready_line_150<R>(f: &std::sync::mpsc::Receiver<R>) -> bool {
        match f.try_recv() {
            Ok(_) => true,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => true,
            Err(std::sync::mpsc::TryRecvError::Empty) => false,
        }
    }

    /// Stub for `test_flag` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:160`.
    pub fn test_flag_line_160(flags: i32, flag: i32) -> bool {
        (flags & flag) == flag
    }

    /// Stub for `starts_with` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:164`.
    pub fn starts_with_line_164(str_: &str, prefix: &str) -> bool {
        str_.starts_with(prefix)
    }

    /// Stub for `ends_with` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:171`.
    pub fn ends_with_line_171(str_: &str, suffix: &str) -> bool {
        str_.ends_with(suffix)
    }

    /// Stub for `trim` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:181`.
    pub fn trim_line_181(s: String) -> String {
        s.trim().to_string()
    }

    /// Stub for `replace_all` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:198`.
    pub fn replace_all_line_198(str_: &str, from: &str, to: &str) -> String {
        if from.is_empty() {
            str_.to_string()
        } else {
            str_.replace(from, to)
        }
    }
}

pub mod field_conv {
    use super::Placeholder;

    /// Stub for `skip_spaces` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:34`.
    pub fn skip_spaces_line_34(s: &str, mut pos: usize) -> usize {
        let bytes = s.as_bytes();
        while pos < bytes.len()
            && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r')
        {
            pos += 1;
        }
        pos
    }

    /// Stub for `skip_spaces_and_newlines` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:38`.
    pub fn skip_spaces_and_newlines_line_38(s: &str, mut pos: usize) -> usize {
        let bytes = s.as_bytes();
        while pos < bytes.len()
            && (bytes[pos] == b' '
                || bytes[pos] == b'\t'
                || bytes[pos] == b'\r'
                || bytes[pos] == b'\n')
        {
            pos += 1;
        }
        pos
    }

    /// Stub for `bump_to_next_line` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:48`.
    pub fn bump_to_next_line_line_48(s: &str, mut pos: usize) -> usize {
        let bytes = s.as_bytes();
        while pos < bytes.len() && bytes[pos] != b'\n' {
            pos += 1;
        }
        if pos < bytes.len() {
            pos += 1;
        }
        pos
    }

    /// Stub for `read_int_from_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:72`.
    pub fn read_int_from_chars_line_72(
        s: &str,
        pos: usize,
        end: usize,
    ) -> Result<(usize, i64), String> {
        read_int_line_140(&s[..end.min(s.len())], pos)
    }

    /// Stub for `read_int_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:85`.
    pub fn read_int_fallback_line_85(
        s: &str,
        pos: usize,
        end: usize,
    ) -> Result<(usize, i64), String> {
        let bounded = &s[..end.min(s.len())];
        let mut next = pos;
        let bytes = bounded.as_bytes();
        while next < bytes.len()
            && (bytes[next] == b' '
                || bytes[next] == b'\t'
                || bytes[next] == b'\n'
                || bytes[next] == b'\r'
                || bytes[next] == 0x0b
                || bytes[next] == 0x0c)
        {
            next += 1;
        }
        read_int_line_140(bounded, next)
    }

    /// Stub for `read_int_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:101`.
    pub fn read_int_fallback_line_101(
        s: &str,
        pos: usize,
        end: usize,
    ) -> Result<(usize, u64), String> {
        let mut next = pos;
        let bounded = &s[..end.min(s.len())];
        let bytes = bounded.as_bytes();
        while next < bytes.len()
            && (bytes[next] == b' '
                || bytes[next] == b'\t'
                || bytes[next] == b'\n'
                || bytes[next] == b'\r'
                || bytes[next] == 0x0b
                || bytes[next] == 0x0c)
        {
            next += 1;
        }
        let sign_pos = next;
        let negative = next < bytes.len() && bytes[next] == b'-';
        if next < bytes.len() && (bytes[next] == b'+' || bytes[next] == b'-') {
            next += 1;
        }
        while next < bytes.len() && bytes[next].is_ascii_digit() {
            next += 1;
        }
        if next == sign_pos
            || (next == sign_pos + 1 && (bytes[sign_pos] == b'+' || bytes[sign_pos] == b'-'))
        {
            return Err("Invalid integer value.".to_string());
        }
        let digits_start = if bytes[sign_pos] == b'+' || bytes[sign_pos] == b'-' {
            sign_pos + 1
        } else {
            sign_pos
        };
        bounded[digits_start..next]
            .parse::<u64>()
            .map(|value| {
                if negative {
                    (next, 0u64.wrapping_sub(value))
                } else {
                    (next, value)
                }
            })
            .map_err(|_| "Integer out of range.".to_string())
    }

    /// Stub for `read_int_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:122`.
    pub fn read_int_fallback_line_122(
        s: &str,
        pos: usize,
        end: usize,
    ) -> Result<(usize, i64), String> {
        read_int_fallback_line_85(s, pos, end)
    }

    /// Stub for `read_int` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:140`.
    pub fn read_int_line_140(s: &str, pos: usize) -> Result<(usize, i64), String> {
        let mut end = pos;
        let bytes = s.as_bytes();
        if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
            end += 1;
        }
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        if end == pos || (end == pos + 1 && (bytes[pos] == b'+' || bytes[pos] == b'-')) {
            return Err("Invalid integer value.".to_string());
        }
        s[pos..end]
            .parse::<i64>()
            .map(|value| (end, value))
            .map_err(|_| "Integer out of range.".to_string())
    }

    /// Stub for `read_float_fast_float` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:153`.
    pub fn read_float_fast_float_line_153(
        s: &str,
        pos: usize,
        end: usize,
        throw_out_of_range: bool,
    ) -> Result<(usize, f64), String> {
        let bounded = &s[..end.min(s.len())];
        let (next, value) = read_float_fallback_line_196(bounded, pos)?;
        if throw_out_of_range && value.is_infinite() {
            let token = &bounded.as_bytes()[pos..next];
            let start = if !token.is_empty() && (token[0] == b'+' || token[0] == b'-') {
                1
            } else {
                0
            };
            let explicit_infinity = (token.len() == start + "inf".len()
                && token[start].to_ascii_lowercase() == b'i'
                && token[start + 1].to_ascii_lowercase() == b'n'
                && token[start + 2].to_ascii_lowercase() == b'f')
                || (token.len() == start + "infinity".len()
                    && token[start].to_ascii_lowercase() == b'i'
                    && token[start + 1].to_ascii_lowercase() == b'n'
                    && token[start + 2].to_ascii_lowercase() == b'f'
                    && token[start + 3].to_ascii_lowercase() == b'i'
                    && token[start + 4].to_ascii_lowercase() == b'n'
                    && token[start + 5].to_ascii_lowercase() == b'i'
                    && token[start + 6].to_ascii_lowercase() == b't'
                    && token[start + 7].to_ascii_lowercase() == b'y');
            if !explicit_infinity {
                return Err("Floating-point value out of range.".to_string());
            }
        }
        Ok((next, value))
    }

    /// Stub for `read_float_from_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:175`.
    pub fn read_float_from_chars_line_175(
        s: &str,
        pos: usize,
        end: usize,
        throw_out_of_range: bool,
    ) -> Result<(usize, f64), String> {
        let bounded = &s[..end.min(s.len())];
        let (next, value) = read_float_fallback_line_196(bounded, pos)?;
        if throw_out_of_range && value.is_infinite() {
            let token = &bounded.as_bytes()[pos..next];
            let start = if !token.is_empty() && (token[0] == b'+' || token[0] == b'-') {
                1
            } else {
                0
            };
            let explicit_infinity = (token.len() == start + "inf".len()
                && token[start].to_ascii_lowercase() == b'i'
                && token[start + 1].to_ascii_lowercase() == b'n'
                && token[start + 2].to_ascii_lowercase() == b'f')
                || (token.len() == start + "infinity".len()
                    && token[start].to_ascii_lowercase() == b'i'
                    && token[start + 1].to_ascii_lowercase() == b'n'
                    && token[start + 2].to_ascii_lowercase() == b'f'
                    && token[start + 3].to_ascii_lowercase() == b'i'
                    && token[start + 4].to_ascii_lowercase() == b'n'
                    && token[start + 5].to_ascii_lowercase() == b'i'
                    && token[start + 6].to_ascii_lowercase() == b't'
                    && token[start + 7].to_ascii_lowercase() == b'y');
            if !explicit_infinity {
                return Err("Floating-point overflow".to_string());
            }
        }
        Ok((next, value))
    }

    /// Stub for `read_float_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:196`.
    pub fn read_float_fallback_line_196(s: &str, pos: usize) -> Result<(usize, f64), String> {
        let bytes = s.as_bytes();
        let mut end = pos;
        while end < bytes.len()
            && (bytes[end] == b' '
                || bytes[end] == b'\t'
                || bytes[end] == b'\n'
                || bytes[end] == b'\r'
                || bytes[end] == 0x0b
                || bytes[end] == 0x0c)
        {
            end += 1;
        }
        let parse_start = end;
        if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
            end += 1;
        }

        let value_start = end;
        if value_start + "infinity".len() <= bytes.len()
            && bytes[value_start].to_ascii_lowercase() == b'i'
            && bytes[value_start + 1].to_ascii_lowercase() == b'n'
            && bytes[value_start + 2].to_ascii_lowercase() == b'f'
            && bytes[value_start + 3].to_ascii_lowercase() == b'i'
            && bytes[value_start + 4].to_ascii_lowercase() == b'n'
            && bytes[value_start + 5].to_ascii_lowercase() == b'i'
            && bytes[value_start + 6].to_ascii_lowercase() == b't'
            && bytes[value_start + 7].to_ascii_lowercase() == b'y'
        {
            end = value_start + "infinity".len();
        } else if value_start + "inf".len() <= bytes.len()
            && bytes[value_start].to_ascii_lowercase() == b'i'
            && bytes[value_start + 1].to_ascii_lowercase() == b'n'
            && bytes[value_start + 2].to_ascii_lowercase() == b'f'
        {
            end = value_start + "inf".len();
        } else if value_start + "nan".len() <= bytes.len()
            && bytes[value_start].to_ascii_lowercase() == b'n'
            && bytes[value_start + 1].to_ascii_lowercase() == b'a'
            && bytes[value_start + 2].to_ascii_lowercase() == b'n'
        {
            end = value_start + "nan".len();
            if end < bytes.len() && bytes[end] == b'(' {
                let payload_start = end;
                end += 1;
                while end < bytes.len()
                    && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_')
                {
                    end += 1;
                }
                if end < bytes.len() && bytes[end] == b')' {
                    end += 1;
                } else {
                    end = payload_start;
                }
            }
        } else {
            let mantissa_start = end;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            let digits_before_dot = end > mantissa_start;
            if end < bytes.len() && bytes[end] == b'.' {
                end += 1;
                let fraction_start = end;
                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }
                if !digits_before_dot && end == fraction_start {
                    return Err("Invalid floating-point value.".to_string());
                }
            } else if !digits_before_dot {
                return Err("Invalid floating-point value.".to_string());
            }

            if end < bytes.len() && (bytes[end] == b'e' || bytes[end] == b'E') {
                let exponent_marker = end;
                end += 1;
                if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
                    end += 1;
                }
                let exponent_digits = end;
                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }
                if end == exponent_digits {
                    end = exponent_marker;
                }
            }
        }
        if end == parse_start {
            return Err("Invalid floating-point value.".to_string());
        }
        let token = &s[parse_start..end];
        let token_bytes = token.as_bytes();
        let start = if !token_bytes.is_empty() && (token_bytes[0] == b'+' || token_bytes[0] == b'-')
        {
            1
        } else {
            0
        };
        if token_bytes.len() > start + "nan".len()
            && token_bytes[start].to_ascii_lowercase() == b'n'
            && token_bytes[start + 1].to_ascii_lowercase() == b'a'
            && token_bytes[start + 2].to_ascii_lowercase() == b'n'
            && token_bytes[start + 3] == b'('
        {
            return Ok((end, f64::NAN));
        }
        token
            .parse::<f64>()
            .map(|value| (end, value))
            .map_err(|_| "Invalid floating-point value.".to_string())
    }

    /// Stub for `read_float_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:216`.
    pub fn read_float_fallback_line_216(s: &str, pos: usize) -> Result<(usize, f32), String> {
        let (next, value) = read_float_fallback_line_196(s, pos)?;
        if value.is_finite()
            && value != 0.0
            && (value.abs() > f32::MAX as f64 || value.abs() < f32::MIN_POSITIVE as f64)
        {
            return Err("Floating-point value out of range.".to_string());
        }
        Ok((next, value as f32))
    }

    /// Stub for `read_float` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:234`.
    pub fn read_float_line_234(s: &str, pos: usize) -> Result<(usize, f64), String> {
        read_float_fallback_line_196(s, pos)
    }

    /// Stub for `read_float_from_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:257`.
    pub fn read_float_from_chars_line_257(
        s: &str,
        pos: usize,
        end: usize,
        throw_out_of_range: bool,
    ) -> Result<(usize, f64), String> {
        let bounded = &s[..end.min(s.len())];
        let (next, value) = read_float_fallback_line_280(bounded, pos)?;
        if throw_out_of_range && value.is_infinite() {
            let token = &bounded.as_bytes()[pos..next];
            let start = if !token.is_empty() && (token[0] == b'+' || token[0] == b'-') {
                1
            } else {
                0
            };
            let explicit_infinity = (token.len() == start + "inf".len()
                && token[start].to_ascii_lowercase() == b'i'
                && token[start + 1].to_ascii_lowercase() == b'n'
                && token[start + 2].to_ascii_lowercase() == b'f')
                || (token.len() == start + "infinity".len()
                    && token[start].to_ascii_lowercase() == b'i'
                    && token[start + 1].to_ascii_lowercase() == b'n'
                    && token[start + 2].to_ascii_lowercase() == b'f'
                    && token[start + 3].to_ascii_lowercase() == b'i'
                    && token[start + 4].to_ascii_lowercase() == b'n'
                    && token[start + 5].to_ascii_lowercase() == b'i'
                    && token[start + 6].to_ascii_lowercase() == b't'
                    && token[start + 7].to_ascii_lowercase() == b'y');
            if !explicit_infinity {
                return Err("Floating-point value out of range.".to_string());
            }
        }
        Ok((next, value))
    }

    /// Stub for `read_float_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:280`.
    pub fn read_float_fallback_line_280(s: &str, pos: usize) -> Result<(usize, f64), String> {
        read_float_fallback_line_196(s, pos)
    }

    /// Stub for `read_float` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:297`.
    pub fn read_float_line_297(s: &str, pos: usize) -> Result<(usize, f64), String> {
        read_float_fallback_line_280(s, pos)
    }

    /// Stub for `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:312`.
    pub fn read_value_line_312(pos: usize) -> usize {
        pos
    }

    /// Stub for `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:317`.
    pub fn read_value_line_317(s: &str, pos: usize) -> Result<(usize, i64), String> {
        read_int_line_140(s, pos)
    }

    /// Stub for `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:321`.
    pub fn read_value_line_321(s: &str, pos: usize) -> Result<(usize, bool), String> {
        let (next, parsed) = read_float_line_234(s, pos)?;
        Ok((next, parsed != 0.0))
    }

    /// Stub for `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:329`.
    pub fn read_value_line_329(s: &str, pos: usize) -> Result<(usize, f64), String> {
        read_float_line_234(s, pos)
    }

    /// Stub for `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:334`.
    pub fn read_value_line_334(s: &str, pos: usize) -> Result<(usize, (f64, f64)), String> {
        let (pos, real) = read_float_line_234(s, pos)?;
        let pos = skip_spaces_line_34(s, pos);
        let (pos, imaginary) = read_float_line_234(s, pos)?;
        Ok((pos, (real, imaginary)))
    }

    /// Stub for `complex_conjugate` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:347`.
    pub fn complex_conjugate_line_347(value: (f64, f64)) -> (f64, f64) {
        (value.0, -value.1)
    }

    /// Stub for `complex_conjugate` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:352`.
    pub fn complex_conjugate_line_352<T>(value: T) -> T {
        value
    }

    /// Stub for `int_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:367`.
    pub fn int_to_string_line_367<T: ToString>(value: T) -> String {
        value.to_string()
    }

    /// Stub for `int_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:382`.
    pub fn int_to_string_line_382<T: ToString>(value: T) -> String {
        value.to_string()
    }

    /// Stub for `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:387`.
    pub fn value_to_string_line_387() -> String {
        String::new()
    }

    /// Stub for `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:391`.
    pub fn value_to_string_line_391(value: bool) -> String {
        if value {
            "1".to_string()
        } else {
            "0".to_string()
        }
    }

    /// Stub for `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:396`.
    pub fn value_to_string_line_396<T: ToString>(value: T) -> String {
        int_to_string_line_367(value)
    }

    /// Stub for `value_to_string_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:404`.
    pub fn value_to_string_fallback_line_404(value: f64, precision: i32) -> String {
        if precision < 0 {
            if value.is_nan() {
                "nan".to_string()
            } else if value.is_infinite() {
                if value.is_sign_negative() {
                    "-inf".to_string()
                } else {
                    "inf".to_string()
                }
            } else {
                format!("{value:.6}")
            }
        } else {
            if !value.is_finite() {
                return value.to_string();
            }
            let significant = precision.max(1) as usize;
            if value == 0.0 {
                return "0".to_string();
            }
            let exponent = value.abs().log10().floor() as i32;
            let mut ret = if exponent < -4 || exponent >= significant as i32 {
                let mut scientific = format!("{:.*e}", significant - 1, value);
                if let Some((mantissa, exponent)) = scientific.split_once('e') {
                    let mut mantissa = mantissa.to_string();
                    if let Some(dot) = mantissa.find('.') {
                        while mantissa.ends_with('0') {
                            mantissa.pop();
                        }
                        if mantissa.len() == dot + 1 {
                            mantissa.pop();
                        }
                    }
                    let exponent_value = exponent.parse::<i32>().unwrap_or(0);
                    scientific = format!("{mantissa}e{exponent_value:+}");
                }
                scientific
            } else {
                let decimals = (significant as i32 - exponent - 1).max(0) as usize;
                format!("{value:.decimals$}")
            };
            if let Some(dot) = ret.find('.') {
                while ret.ends_with('0') {
                    ret.pop();
                }
                if ret.len() == dot + 1 {
                    ret.pop();
                }
            }
            if ret == "-0" {
                "0".to_string()
            } else {
                ret
            }
        }
    }

    /// Stub for `value_to_string_dragonbox` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:438`.
    pub fn value_to_string_dragonbox_line_438(value: f32) -> String {
        value.to_string()
    }

    /// Stub for `value_to_string_dragonbox` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:452`.
    pub fn value_to_string_dragonbox_line_452(value: f64) -> String {
        value.to_string()
    }

    /// Stub for `value_to_string_ryu` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:468`.
    pub fn value_to_string_ryu_line_468(value: f32, precision: i32) -> String {
        let mut ret = if precision < 0 {
            value.to_string()
        } else {
            let adjusted = if precision > 0 {
                precision - 1
            } else {
                precision
            };
            format!("{:.*e}", adjusted.max(0) as usize, value as f64)
        };
        if precision < 0 && ret.ends_with("E0") {
            ret.truncate(ret.len() - 2);
        }
        if precision >= 0 && ret.ends_with("e+00") {
            ret.truncate(ret.len() - 4);
        }
        ret
    }

    /// Stub for `value_to_string_ryu` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:501`.
    pub fn value_to_string_ryu_line_501(value: f64, precision: i32) -> String {
        let mut ret = if precision < 0 {
            value.to_string()
        } else {
            let adjusted = if precision > 0 {
                precision - 1
            } else {
                precision
            };
            format!("{:.*e}", adjusted.max(0) as usize, value)
        };
        if precision < 0 && ret.ends_with("E0") {
            ret.truncate(ret.len() - 2);
        }
        if precision >= 0 && ret.ends_with("e+00") {
            ret.truncate(ret.len() - 4);
        }
        ret
    }

    /// Stub for `value_to_string_to_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:537`.
    pub fn value_to_string_to_chars_line_537(value: f64, precision: i32) -> String {
        if precision < 0 {
            value.to_string()
        } else {
            value_to_string_fallback_line_404(value, precision)
        }
    }

    /// Stub for `value_to_string_to_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:558`.
    pub fn value_to_string_to_chars_line_558(value: f64, precision: i32) -> String {
        if precision < 0 {
            value.to_string()
        } else {
            value_to_string_fallback_line_404(value, precision)
        }
    }

    /// Stub for `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:584`.
    pub fn value_to_string_line_584(value: f64, precision: i32) -> String {
        value_to_string_fallback_line_404(value, precision)
    }

    /// Stub for `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:601`.
    pub fn value_to_string_line_601(value: f64, precision: i32) -> String {
        if precision < 0 {
            let dragonbox = value_to_string_dragonbox_line_452(value);
            if !dragonbox.is_empty() {
                return dragonbox;
            }
        }
        let to_chars = value_to_string_to_chars_line_537(value, precision);
        if !to_chars.is_empty() {
            return to_chars;
        }
        let ryu = value_to_string_ryu_line_501(value, precision);
        if !ryu.is_empty() {
            return ryu;
        }
        value_to_string_fallback_line_404(value, precision)
    }

    /// Stub for `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:630`.
    pub fn value_to_string_line_630(value: (f64, f64), precision: i32) -> String {
        format!(
            "{} {}",
            value_to_string_line_601(value.0, precision),
            value_to_string_line_601(value.1, precision)
        )
    }

    /// Stub for `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:638`.
    pub fn value_to_string_line_638<T: ToString>(value: T, _precision: i32) -> String {
        value.to_string()
    }
}

pub mod formatters {
    use super::field_conv;
    use super::types::{
        field_type, format_type, matrix_market_header, storage_order, symmetry_type, write_options,
    };
    use super::Placeholder;

    pub trait write_value_type: Clone {
        fn field_type() -> field_type;
        fn is_pattern() -> bool {
            false
        }
        fn as_fields(&self) -> Vec<f64>;
    }

    impl write_value_type for f64 {
        fn field_type() -> field_type {
            field_type::real
        }

        fn as_fields(&self) -> Vec<f64> {
            vec![*self]
        }
    }

    impl write_value_type for (f64, f64) {
        fn field_type() -> field_type {
            field_type::complex
        }

        fn as_fields(&self) -> Vec<f64> {
            vec![self.0, self.1]
        }
    }

    impl write_value_type for () {
        fn field_type() -> field_type {
            field_type::pattern
        }

        fn is_pattern() -> bool {
            true
        }

        fn as_fields(&self) -> Vec<f64> {
            Vec::new()
        }
    }

    impl write_value_type for Vec<f64> {
        fn field_type() -> field_type {
            field_type::real
        }

        fn as_fields(&self) -> Vec<f64> {
            self.clone()
        }
    }

    /// Original class `line_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:18`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct line_formatter {
        pub header: matrix_market_header,
        pub options: write_options,
    }

    /// Original class `vector_line_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:77`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct vector_line_formatter {
        pub header: matrix_market_header,
        pub options: write_options,
    }

    /// Original class `triplet_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:118`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct triplet_formatter<V = Vec<f64>> {
        pub line_formatter: line_formatter,
        pub rows: Vec<i64>,
        pub cols: Vec<i64>,
        pub values: Vec<V>,
        pub cursor: usize,
    }

    /// Original class `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:138`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct chunk<V = Vec<f64>> {
        pub line_formatter: line_formatter,
        pub rows: Vec<i64>,
        pub cols: Vec<i64>,
        pub values: Vec<V>,
    }

    /// Original class `csc_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:200`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct csc_formatter<V = Vec<f64>> {
        pub line_formatter: line_formatter,
        pub ptrs: Vec<usize>,
        pub ptr_iter: usize,
        pub inds: Vec<i64>,
        pub values: Vec<V>,
        pub transpose: bool,
        pub nnz_per_column: f64,
    }

    /// Original class `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:225`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct chunk_line_225<V = Vec<f64>> {
        pub line_formatter: line_formatter,
        pub ptrs: Vec<usize>,
        pub ptr_iter: usize,
        pub ptr_end: usize,
        pub inds: Vec<i64>,
        pub values: Vec<V>,
        pub transpose: bool,
    }

    /// Original class `array_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:312`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct array_formatter<V = Vec<f64>> {
        pub line_formatter: line_formatter,
        pub values: Vec<V>,
        pub order: storage_order,
        pub nrows: i64,
        pub ncols: i64,
        pub cur_col: i64,
    }

    /// Original class `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:321`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct chunk_line_321<V = Vec<f64>> {
        pub line_formatter: line_formatter,
        pub values: Vec<V>,
        pub order: storage_order,
        pub nrows: i64,
        pub ncols: i64,
        pub cur_col: i64,
    }

    /// Original class `dense_2d_call_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:370`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct dense_2d_call_formatter<V = Vec<f64>> {
        pub line_formatter: line_formatter,
        pub values: Vec<V>,
        pub nrows: i64,
        pub ncols: i64,
        pub col_iter: i64,
    }

    /// Original class `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:379`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct chunk_line_379<V = Vec<f64>> {
        pub line_formatter: line_formatter,
        pub values: Vec<V>,
        pub nrows: i64,
        pub ncols: i64,
        pub col_iter: i64,
        pub col_end: i64,
    }

    /// Stub for `line_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:20`.
    pub fn line_formatter_line_20(
        header: matrix_market_header,
        options: write_options,
    ) -> line_formatter {
        line_formatter { header, options }
    }

    /// Stub for `coord_matrix` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:23`.
    pub fn coord_matrix_line_23(lf: &line_formatter, row: i64, col: i64, val: &[f64]) -> String {
        if lf.header.format == format_type::array {
            return array_matrix_line_52(lf, row, col, val);
        }
        let mut line = String::new();
        line.push_str(&field_conv::int_to_string_line_367(row + 1));
        line.push(' ');
        line.push_str(&field_conv::int_to_string_line_367(col + 1));
        if lf.header.field != field_type::pattern {
            line.push(' ');
            if val.len() == 2 {
                line.push_str(&field_conv::value_to_string_line_630(
                    (val[0], val[1]),
                    lf.options.precision as i32,
                ));
            } else if let Some(value) = val.first() {
                line.push_str(&field_conv::value_to_string_line_601(
                    *value,
                    lf.options.precision as i32,
                ));
            }
        }
        line.push('\n');
        line
    }

    /// Stub for `coord_matrix_pattern` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:42`.
    pub fn coord_matrix_pattern_line_42(row: i64, col: i64) -> String {
        format!("{} {}\n", row + 1, col + 1)
    }

    /// Stub for `array_matrix` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:52`.
    pub fn array_matrix_line_52(lf: &line_formatter, row: i64, col: i64, val: &[f64]) -> String {
        if lf.header.symmetry != symmetry_type::general {
            if row < col {
                return String::new();
            }
            if lf.header.symmetry == symmetry_type::skew_symmetric && row == col {
                return String::new();
            }
        }
        let mut ret = if val.len() == 2 {
            field_conv::value_to_string_line_630((val[0], val[1]), lf.options.precision as i32)
        } else {
            field_conv::value_to_string_line_601(
                *val.first().unwrap_or(&0.0),
                lf.options.precision as i32,
            )
        };
        ret.push('\n');
        ret
    }

    /// Stub for `vector_line_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:79`.
    pub fn vector_line_formatter_line_79(
        header: matrix_market_header,
        options: write_options,
    ) -> vector_line_formatter {
        vector_line_formatter { header, options }
    }

    /// Stub for `coord_matrix` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:81`.
    pub fn coord_matrix_line_81(
        lf: &vector_line_formatter,
        row: i64,
        _col: i64,
        val: &[f64],
    ) -> String {
        let mut line = String::new();
        line.push_str(&field_conv::int_to_string_line_367(row + 1));
        if lf.header.field != field_type::pattern {
            line.push(' ');
            if val.len() == 2 {
                line.push_str(&field_conv::value_to_string_line_630(
                    (val[0], val[1]),
                    lf.options.precision as i32,
                ));
            } else if let Some(value) = val.first() {
                line.push_str(&field_conv::value_to_string_line_601(
                    *value,
                    lf.options.precision as i32,
                ));
            }
        }
        line.push('\n');
        line
    }

    /// Stub for `coord_matrix_pattern` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:94`.
    pub fn coord_matrix_pattern_line_94(row: i64, _col: i64) -> String {
        format!("{}\n", row + 1)
    }

    /// Stub for `triplet_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:120`.
    pub fn triplet_formatter_line_120<V: write_value_type>(
        lf: line_formatter,
        rows: Vec<i64>,
        cols: Vec<i64>,
        values: Vec<V>,
    ) -> Result<triplet_formatter<V>, super::fast_matrix_market::invalid_argument> {
        if rows.len() != cols.len() || (!values.is_empty() && rows.len() != values.len()) {
            return Err(super::fast_matrix_market::invalid_argument_line_77(
                "Row, column, and value ranges must have equal length.".to_string(),
            ));
        }
        Ok(triplet_formatter {
            line_formatter: lf,
            rows,
            cols,
            values,
            cursor: 0,
        })
    }

    /// Stub for `has_next` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:134`.
    pub fn has_next_line_134<V>(formatter: &triplet_formatter<V>) -> bool {
        formatter.cursor != formatter.rows.len()
    }

    /// Stub for `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:140`.
    pub fn chunk_line_140<V: write_value_type>(
        lf: line_formatter,
        rows: Vec<i64>,
        cols: Vec<i64>,
        values: Vec<V>,
    ) -> chunk<V> {
        chunk {
            line_formatter: lf,
            rows,
            cols,
            values,
        }
    }

    /// Stub for `operator()` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:149`.
    pub fn operator_line_149<V: write_value_type>(c: &chunk<V>) -> String {
        let mut out = String::new();
        for i in 0..c.rows.len() {
            if V::is_pattern() || c.values.is_empty() {
                out.push_str(&coord_matrix_pattern_line_42(c.rows[i], c.cols[i]));
            } else {
                let value = c.values[i].as_fields();
                out.push_str(&coord_matrix_line_23(
                    &c.line_formatter,
                    c.rows[i],
                    c.cols[i],
                    &value,
                ));
            }
        }
        out
    }

    /// Stub for `next_chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:171`.
    pub fn next_chunk_line_171<V: write_value_type>(
        formatter: &mut triplet_formatter<V>,
        options: &write_options,
    ) -> chunk<V> {
        let chunk_size = std::cmp::min(
            options.chunk_size_values.max(0) as usize,
            formatter.rows.len().saturating_sub(formatter.cursor),
        );
        let start = formatter.cursor;
        let end = start + chunk_size;
        formatter.cursor = end;
        chunk_line_140(
            formatter.line_formatter.clone(),
            formatter.rows[start..end].to_vec(),
            formatter.cols[start..end].to_vec(),
            if formatter.values.is_empty() {
                Vec::new()
            } else {
                formatter.values[start..end].to_vec()
            },
        )
    }

    /// Stub for `csc_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:202`.
    pub fn csc_formatter_line_202<V: write_value_type>(
        lf: line_formatter,
        ptrs: Vec<usize>,
        inds: Vec<i64>,
        values: Vec<V>,
        transpose: bool,
    ) -> Result<csc_formatter<V>, super::fast_matrix_market::invalid_argument> {
        if ptrs.len() < 2 {
            return Err(super::fast_matrix_market::invalid_argument_line_77(
                "Pointer range must contain at least one column.".to_string(),
            ));
        }
        if !values.is_empty() && inds.len() != values.len() {
            return Err(super::fast_matrix_market::invalid_argument_line_77(
                "Index and value ranges must have equal length.".to_string(),
            ));
        }
        let num_columns = ptrs.len() - 1;
        let nnz_per_column = inds.len() as f64 / num_columns as f64;
        Ok(csc_formatter {
            line_formatter: lf,
            ptrs,
            ptr_iter: 0,
            inds,
            values,
            transpose,
            nnz_per_column,
        })
    }

    /// Stub for `has_next` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:221`.
    pub fn has_next_line_221<V>(formatter: &csc_formatter<V>) -> bool {
        formatter.ptr_iter + 1 < formatter.ptrs.len()
    }

    /// Stub for `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:227`.
    pub fn chunk_line_227<V: write_value_type>(
        lf: line_formatter,
        ptrs: Vec<usize>,
        ptr_iter: usize,
        ptr_end: usize,
        inds: Vec<i64>,
        values: Vec<V>,
        transpose: bool,
    ) -> chunk_line_225<V> {
        chunk_line_225 {
            line_formatter: lf,
            ptrs,
            ptr_iter,
            ptr_end,
            inds,
            values,
            transpose,
        }
    }

    /// Stub for `operator()` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:238`.
    pub fn operator_line_238<V: write_value_type>(c: &chunk_line_225<V>) -> String {
        let mut out = String::new();
        for column_number in c.ptr_iter..c.ptr_end {
            let row_start = c.ptrs[column_number];
            let row_end = c.ptrs[column_number + 1];
            for idx in row_start..row_end {
                let mut lf_row = c.inds[idx];
                let mut lf_col = column_number as i64;
                if c.transpose {
                    std::mem::swap(&mut lf_row, &mut lf_col);
                }
                if V::is_pattern() || c.values.is_empty() {
                    out.push_str(&coord_matrix_pattern_line_42(lf_row, lf_col));
                } else {
                    let value = c.values[idx].as_fields();
                    out.push_str(&coord_matrix_line_23(
                        &c.line_formatter,
                        lf_row,
                        lf_col,
                        &value,
                    ));
                }
            }
        }
        out
    }

    /// Stub for `next_chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:282`.
    pub fn next_chunk_line_282<V: write_value_type>(
        formatter: &mut csc_formatter<V>,
        options: &write_options,
    ) -> chunk_line_225<V> {
        let mut num_columns =
            (options.chunk_size_values as f64 / formatter.nnz_per_column + 1.0) as usize;
        num_columns = num_columns.min(formatter.ptrs.len().saturating_sub(formatter.ptr_iter + 1));
        let ptr_end = formatter.ptr_iter + num_columns;
        let c = chunk_line_227(
            formatter.line_formatter.clone(),
            formatter.ptrs.clone(),
            formatter.ptr_iter,
            ptr_end,
            formatter.inds.clone(),
            formatter.values.clone(),
            formatter.transpose,
        );
        formatter.ptr_iter = ptr_end;
        c
    }

    /// Stub for `array_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:314`.
    pub fn array_formatter_line_314<V: write_value_type>(
        lf: line_formatter,
        values: Vec<V>,
        order: storage_order,
        nrows: i64,
        ncols: i64,
    ) -> array_formatter<V> {
        array_formatter {
            line_formatter: lf,
            values,
            order,
            nrows,
            ncols,
            cur_col: 0,
        }
    }

    /// Stub for `has_next` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:317`.
    pub fn has_next_line_317<V>(formatter: &array_formatter<V>) -> bool {
        formatter.cur_col != formatter.ncols
    }

    /// Stub for `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:323`.
    pub fn chunk_line_323<V: write_value_type>(
        lf: line_formatter,
        values: Vec<V>,
        order: storage_order,
        nrows: i64,
        ncols: i64,
        cur_col: i64,
    ) -> chunk_line_321<V> {
        chunk_line_321 {
            line_formatter: lf,
            values,
            order,
            nrows,
            ncols,
            cur_col,
        }
    }

    /// Stub for `operator()` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:326`.
    pub fn operator_line_326<V: write_value_type>(c: &chunk_line_321<V>) -> String {
        let mut out = String::new();
        for row in 0..c.nrows {
            let offset = if c.order == storage_order::row_major {
                row * c.ncols + c.cur_col
            } else {
                c.cur_col * c.nrows + row
            };
            if let Some(value) = c.values.get(offset as usize) {
                let value = value.as_fields();
                out.push_str(&array_matrix_line_52(
                    &c.line_formatter,
                    row,
                    c.cur_col,
                    &value,
                ));
            }
        }
        out
    }

    /// Stub for `next_chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:351`.
    pub fn next_chunk_line_351<V: write_value_type>(
        formatter: &mut array_formatter<V>,
        _options: &write_options,
    ) -> chunk_line_321<V> {
        let c = chunk_line_323(
            formatter.line_formatter.clone(),
            formatter.values.clone(),
            formatter.order,
            formatter.nrows,
            formatter.ncols,
            formatter.cur_col,
        );
        formatter.cur_col += 1;
        c
    }

    /// Stub for `dense_2d_call_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:372`.
    pub fn dense_2d_call_formatter_line_372<V: write_value_type>(
        lf: line_formatter,
        values: Vec<V>,
        nrows: i64,
        ncols: i64,
    ) -> dense_2d_call_formatter<V> {
        dense_2d_call_formatter {
            line_formatter: lf,
            values,
            nrows,
            ncols,
            col_iter: 0,
        }
    }

    /// Stub for `has_next` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:375`.
    pub fn has_next_line_375<V>(formatter: &dense_2d_call_formatter<V>) -> bool {
        formatter.col_iter < formatter.ncols
    }

    /// Stub for `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:381`.
    pub fn chunk_line_381<V: write_value_type>(
        lf: line_formatter,
        values: Vec<V>,
        nrows: i64,
        ncols: i64,
        col_iter: i64,
        col_end: i64,
    ) -> chunk_line_379<V> {
        chunk_line_379 {
            line_formatter: lf,
            values,
            nrows,
            ncols,
            col_iter,
            col_end,
        }
    }

    /// Stub for `operator()` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:384`.
    pub fn operator_line_384<V: write_value_type>(c: &chunk_line_379<V>) -> String {
        let mut out = String::new();
        for col in c.col_iter..c.col_end {
            for row in 0..c.nrows {
                if let Some(value) = c.values.get((row * c.ncols + col) as usize) {
                    let value = value.as_fields();
                    out.push_str(&array_matrix_line_52(&c.line_formatter, row, col, &value));
                }
            }
        }
        out
    }

    /// Stub for `next_chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:406`.
    pub fn next_chunk_line_406<V: write_value_type>(
        formatter: &mut dense_2d_call_formatter<V>,
        options: &write_options,
    ) -> chunk_line_379<V> {
        let num_columns = ((options.chunk_size_values as f64 / formatter.nrows as f64) as i64 + 1)
            .min(formatter.ncols - formatter.col_iter);
        let col_end = formatter.col_iter + num_columns;
        let c = chunk_line_381(
            formatter.line_formatter.clone(),
            formatter.values.clone(),
            formatter.nrows,
            formatter.ncols,
            formatter.col_iter,
            col_end,
        );
        formatter.col_iter = col_end;
        c
    }
}

pub mod header {
    use super::chunking;
    use super::fast_matrix_market::{self, invalid_argument, invalid_mm};
    use super::field_conv;
    use super::types::{
        field_type, format_type, matrix_market_header, object_type, read_options, symmetry_type,
        write_options,
    };
    use super::Placeholder;
    use std::io::{BufRead, Write};

    pub const K_MATRIX_MARKET_BANNER: &str = "%%MatrixMarket";
    pub const K_MATRIX_MARKET_BANNER2: &str = "%MatrixMarket";

    /// Stub for `parse_enum` at `fast_matrix_market/include/fast_matrix_market/header.hpp:24`.
    pub fn parse_enum_line_24(s: &str, values: &[&str]) -> Result<usize, invalid_argument> {
        let lower = s.to_ascii_lowercase();
        for (index, value) in values.iter().enumerate() {
            if *value == lower {
                return Ok(index);
            }
        }
        Err(fast_matrix_market::invalid_argument_line_77(format!(
            "Invalid value. Must be one of: {}",
            values.join(", ")
        )))
    }

    /// Stub for `is_line_all_spaces` at `fast_matrix_market/include/fast_matrix_market/header.hpp:45`.
    pub fn is_line_all_spaces_line_45(line: &str) -> bool {
        let line = line.strip_suffix('\n').unwrap_or(line);
        chunking::is_all_spaces_line_59(line)
    }

    /// Stub for `strip_trailing_cr` at `fast_matrix_market/include/fast_matrix_market/header.hpp:60`.
    pub fn strip_trailing_cr_line_60(line: &mut String) {
        if line.ends_with('\r') {
            line.pop();
        }
    }

    /// Stub for `get_storage_nnz` at `fast_matrix_market/include/fast_matrix_market/header.hpp:71`.
    pub fn get_storage_nnz_line_71(
        header: &matrix_market_header,
        options: read_options,
    ) -> Result<i64, fast_matrix_market::fmm_error> {
        if header.object == object_type::vector {
            return Ok(header.nnz);
        }

        if header.format == format_type::coordinate {
            if header.symmetry != symmetry_type::general && options.generalize_symmetry {
                return Ok(2 * header.nnz);
            }
            return Ok(header.nnz);
        }

        let diag_count = header.nrows;
        let off_diag_count = header.nrows * header.ncols - diag_count;
        let off_diag_half = off_diag_count / 2;

        if options.generalize_symmetry {
            if header.symmetry == symmetry_type::skew_symmetric {
                Ok(off_diag_count)
            } else {
                Ok(header.nnz)
            }
        } else {
            match header.symmetry {
                symmetry_type::symmetric => Ok(off_diag_half + diag_count),
                symmetry_type::skew_symmetric => Ok(off_diag_half),
                symmetry_type::hermitian => Ok(off_diag_half + diag_count),
                symmetry_type::general => Ok(header.nnz),
            }
        }
    }

    /// Stub for `read_comment` at `fast_matrix_market/include/fast_matrix_market/header.hpp:116`.
    pub fn read_comment_line_116(header: &mut matrix_market_header, line: &str) -> bool {
        if is_line_all_spaces_line_45(line) {
            return true;
        }

        let bytes = line.as_bytes();
        let mut pos = 0usize;
        while pos + 1 < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
            pos += 1;
        }

        if bytes.get(pos) != Some(&b'%') {
            return false;
        }

        pos += 1;
        header.comment.push_str(&line[pos..]);
        header.comment.push('\n');
        true
    }

    /// Stub for `parse_header_enum` at `fast_matrix_market/include/fast_matrix_market/header.hpp:145`.
    pub fn parse_header_enum_line_145(
        s: &str,
        values: &[&str],
        line_num: i64,
    ) -> Result<usize, invalid_mm> {
        let lower = s.to_ascii_lowercase();
        for (index, value) in values.iter().enumerate() {
            if *value == lower {
                return Ok(index);
            }
        }
        Err(fast_matrix_market::invalid_mm_line_55(
            format!("Invalid MatrixMarket header element: {s}"),
            line_num,
        ))
    }

    /// Stub for `read_header` at `fast_matrix_market/include/fast_matrix_market/header.hpp:166`.
    pub fn read_header_line_166<R: BufRead>(
        instream: &mut R,
        header: &mut matrix_market_header,
    ) -> Result<i64, invalid_mm> {
        let mut lines_read = 0i64;
        let mut line = String::new();

        instream.read_line(&mut line).map_err(|_| {
            fast_matrix_market::invalid_mm_line_55(
                "Not a Matrix Market file. Missing banner.".to_string(),
                1,
            )
        })?;
        if line.ends_with('\n') {
            line.pop();
        }
        strip_trailing_cr_line_60(&mut line);
        lines_read += 1;

        if !line.contains("MatrixMarket") {
            return Err(fast_matrix_market::invalid_mm_line_55(
                "Not a Matrix Market file. Missing banner.".to_string(),
                lines_read,
            ));
        }

        let banner_parts: Vec<&str> = line.split_whitespace().collect();
        let banner = banner_parts.get(0).copied().unwrap_or("");
        if banner != K_MATRIX_MARKET_BANNER && banner != K_MATRIX_MARKET_BANNER2 {
            return Err(fast_matrix_market::invalid_mm_line_55(
                "Not a Matrix Market file. Missing banner.".to_string(),
                lines_read,
            ));
        }

        let object = banner_parts.get(1).copied().unwrap_or("");
        let format = banner_parts.get(2).copied().unwrap_or("");
        let field = banner_parts.get(3).copied().unwrap_or("");
        let symmetry = banner_parts.get(4).copied().unwrap_or("");

        header.object = match parse_header_enum_line_145(object, &["matrix", "vector"], lines_read)?
        {
            0 => object_type::matrix,
            _ => object_type::vector,
        };
        header.format =
            match parse_header_enum_line_145(format, &["array", "coordinate"], lines_read)? {
                0 => format_type::array,
                _ => format_type::coordinate,
            };
        header.field = match parse_header_enum_line_145(
            field,
            &[
                "real",
                "double",
                "complex",
                "integer",
                "pattern",
                "unsigned-integer",
            ],
            lines_read,
        )? {
            0 => field_type::real,
            1 => field_type::double_,
            2 => field_type::complex,
            3 => field_type::integer,
            4 => field_type::pattern,
            _ => field_type::unsigned_integer,
        };
        header.symmetry = match parse_header_enum_line_145(
            symmetry,
            &["general", "symmetric", "skew-symmetric", "hermitian"],
            lines_read,
        )? {
            0 => symmetry_type::general,
            1 => symmetry_type::symmetric,
            2 => symmetry_type::skew_symmetric,
            _ => symmetry_type::hermitian,
        };

        loop {
            line.clear();
            let bytes = instream.read_line(&mut line).map_err(|_| {
                fast_matrix_market::invalid_mm_line_55(
                    "Invalid MatrixMarket header: Premature EOF".to_string(),
                    lines_read + 1,
                )
            })?;
            if line.ends_with('\n') {
                line.pop();
            }
            strip_trailing_cr_line_60(&mut line);
            lines_read += 1;

            if bytes == 0 {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Invalid MatrixMarket header: Premature EOF".to_string(),
                    lines_read,
                ));
            }
            if !read_comment_line_116(header, &line) {
                break;
            }
        }

        if header.comment.ends_with('\n') {
            header.comment.pop();
        }

        let mut pos = field_conv::skip_spaces_line_34(&line, 0);
        if header.object == object_type::vector {
            let (next_pos, vector_length) =
                field_conv::read_int_line_140(&line, pos).map_err(|_| {
                    fast_matrix_market::invalid_mm_line_55(
                        "Header dimension line not of length 1".to_string(),
                        lines_read,
                    )
                })?;
            pos = next_pos;
            header.vector_length = vector_length;
            if header.vector_length < 0 {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Vector length can't be negative.".to_string(),
                    lines_read,
                ));
            }
            if header.format == format_type::coordinate {
                pos = field_conv::skip_spaces_line_34(&line, pos);
                let (next_pos, nnz) = field_conv::read_int_line_140(&line, pos).map_err(|_| {
                    fast_matrix_market::invalid_mm_line_55(
                        "Header dimension line not of length 2".to_string(),
                        lines_read,
                    )
                })?;
                pos = next_pos;
                header.nnz = nnz;
            } else {
                header.nnz = header.vector_length;
            }
            let expected_length = if header.format == format_type::coordinate {
                2
            } else {
                1
            };
            pos = field_conv::skip_spaces_line_34(&line, pos);
            if pos != line.len() {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    format!("Header dimension line not of length {expected_length}"),
                    lines_read,
                ));
            }
            header.nrows = header.vector_length;
            header.ncols = 1;
        } else {
            let (next_pos, nrows) = field_conv::read_int_line_140(&line, pos).map_err(|_| {
                fast_matrix_market::invalid_mm_line_55(
                    "Header dimension line not of length 2".to_string(),
                    lines_read,
                )
            })?;
            pos = field_conv::skip_spaces_line_34(&line, next_pos);
            let (next_pos, ncols) = field_conv::read_int_line_140(&line, pos).map_err(|_| {
                fast_matrix_market::invalid_mm_line_55(
                    "Header dimension line not of length 2".to_string(),
                    lines_read,
                )
            })?;
            pos = next_pos;
            header.nrows = nrows;
            header.ncols = ncols;
            if header.nrows < 0 || header.ncols < 0 {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Matrix dimensions can't be negative.".to_string(),
                    lines_read,
                ));
            }
            if header.format == format_type::coordinate {
                pos = field_conv::skip_spaces_line_34(&line, pos);
                let (next_pos, nnz) = field_conv::read_int_line_140(&line, pos).map_err(|_| {
                    fast_matrix_market::invalid_mm_line_55(
                        "Header dimension line not of length 3".to_string(),
                        lines_read,
                    )
                })?;
                pos = next_pos;
                header.nnz = nnz;
                if header.nnz < 0 {
                    return Err(fast_matrix_market::invalid_mm_line_55(
                        "Matrix NNZ can't be negative.".to_string(),
                        lines_read,
                    ));
                }
            } else {
                header.nnz = header.nrows * header.ncols;
            }
            let expected_length = if header.format == format_type::coordinate {
                3
            } else {
                2
            };
            pos = field_conv::skip_spaces_line_34(&line, pos);
            if pos != line.len() {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    format!("Header dimension line not of length {expected_length}"),
                    lines_read,
                ));
            }
            if std::cmp::min(header.nrows, header.ncols) == 1 {
                header.vector_length = std::cmp::max(header.nrows, header.ncols);
            } else {
                header.vector_length = -1;
            }
        }

        header.header_line_count = lines_read;
        Ok(lines_read)
    }

    /// Stub for `write_header` at `fast_matrix_market/include/fast_matrix_market/header.hpp:278`.
    pub fn write_header_line_278<W: Write>(
        os: &mut W,
        header: &matrix_market_header,
        options: write_options,
    ) -> std::io::Result<bool> {
        let object = match header.object {
            object_type::matrix => "matrix",
            object_type::vector => "vector",
        };
        let format = match header.format {
            format_type::array => "array",
            format_type::coordinate => "coordinate",
        };
        let field = match header.field {
            field_type::real => "real",
            field_type::double_ => "double",
            field_type::complex => "complex",
            field_type::integer => "integer",
            field_type::pattern => "pattern",
            field_type::unsigned_integer => "unsigned-integer",
        };
        let symmetry = match header.symmetry {
            symmetry_type::general => "general",
            symmetry_type::symmetric => "symmetric",
            symmetry_type::skew_symmetric => "skew-symmetric",
            symmetry_type::hermitian => "hermitian",
        };

        write!(
            os,
            "{} {} {} {} {}\n",
            K_MATRIX_MARKET_BANNER, object, format, field, symmetry
        )?;
        if !header.comment.is_empty() {
            let write_comment =
                fast_matrix_market::replace_all_line_198(&header.comment, "\n", "\n%");
            write!(os, "%{write_comment}\n")?;
        } else if options.always_comment {
            write!(os, "%\n")?;
        }

        if header.object == object_type::vector {
            write!(os, "{}", header.vector_length)?;
            if header.format == format_type::coordinate {
                write!(os, " {}", header.nnz)?;
            }
        } else {
            write!(os, "{} {}", header.nrows, header.ncols)?;
            if header.format == format_type::coordinate {
                write!(os, " {}", header.nnz)?;
            }
        }
        write!(os, "\n")?;
        Ok(true)
    }
}

pub mod parse_handlers {
    use super::types::storage_order;

    pub trait dense_parse_value: Clone + Default {
        fn add_assign_value(&mut self, value: &Self);
    }

    impl dense_parse_value for f64 {
        fn add_assign_value(&mut self, value: &Self) {
            *self += *value;
        }
    }

    impl dense_parse_value for (f64, f64) {
        fn add_assign_value(&mut self, value: &Self) {
            self.0 += value.0;
            self.1 += value.1;
        }
    }

    impl dense_parse_value for () {
        fn add_assign_value(&mut self, _value: &Self) {}
    }

    impl dense_parse_value for Vec<f64> {
        fn add_assign_value(&mut self, value: &Self) {
            if self.len() < value.len() {
                self.resize(value.len(), 0.0);
            }
            for (idx, value) in value.iter().enumerate() {
                self[idx] += *value;
            }
        }
    }

    /// Original class `tuple_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:42`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct tuple_parse_handler<V = Vec<f64>> {
        /// Original C++ type: `int`.
        pub flags: i64,
        /// Original C++ type: `ITER`.
        pub begin_iter: usize,
        /// Original C++ type: `ITER`.
        pub iter: usize,
        pub tuples: Vec<(i64, i64, V)>,
    }

    /// Original class `triplet_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:70`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct triplet_parse_handler<V = Vec<f64>> {
        /// Original C++ type: `int`.
        pub flags: i64,
        /// Original C++ type: `IT_ITER`.
        pub begin_rows: usize,
        /// Original C++ type: `IT_ITER`.
        pub begin_cols: usize,
        /// Original C++ type: `VT_ITER`.
        pub begin_values: usize,
        /// Original C++ type: `IT_ITER`.
        pub rows: usize,
        /// Original C++ type: `IT_ITER`.
        pub cols: usize,
        /// Original C++ type: `VT_ITER`.
        pub values: usize,
        pub row_values: Vec<i64>,
        pub col_values: Vec<i64>,
        pub entry_values: Vec<V>,
    }

    /// Original class `triplet_pattern_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:120`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct triplet_pattern_parse_handler {
        /// Original C++ type: `int`.
        pub flags: i64,
        /// Original C++ type: `IT_ITER`.
        pub begin_rows: usize,
        /// Original C++ type: `IT_ITER`.
        pub begin_cols: usize,
        /// Original C++ type: `IT_ITER`.
        pub rows: usize,
        /// Original C++ type: `IT_ITER`.
        pub cols: usize,
        pub row_values: Vec<i64>,
        pub col_values: Vec<i64>,
    }

    /// Original class `triplet_calling_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:154`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct triplet_calling_parse_handler<V = Vec<f64>> {
        /// Original C++ type: `int`.
        pub flags: i64,
        /// Original C++ type: `int64_t`.
        pub offset: i64,
        pub row_values: Vec<i64>,
        pub col_values: Vec<i64>,
        pub entry_values: Vec<V>,
    }

    /// Original class `doublet_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:189`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct doublet_parse_handler<V = Vec<f64>> {
        /// Original C++ type: `int`.
        pub flags: i64,
        /// Original C++ type: `IT_ITER`.
        pub begin_index: usize,
        /// Original C++ type: `VT_ITER`.
        pub begin_values: usize,
        /// Original C++ type: `IT_ITER`.
        pub index: usize,
        /// Original C++ type: `VT_ITER`.
        pub values: usize,
        pub index_values: Vec<i64>,
        pub entry_values: Vec<V>,
    }

    /// Original class `dense_2d_call_adding_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:223`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct dense_2d_call_adding_parse_handler<V = Vec<f64>> {
        /// Original C++ type: `int`.
        pub flags: i64,
        pub nrows: i64,
        pub ncols: i64,
        pub values: Vec<V>,
    }

    /// Original class `dense_adding_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:247`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct dense_adding_parse_handler<V = Vec<f64>> {
        /// Original C++ type: `int`.
        pub flags: i64,
        /// Original C++ type: `VT_ITER`.
        pub values: Vec<V>,
        /// Original C++ type: `storage_order`.
        pub order: storage_order,
        /// Original C++ type: `int64_t`.
        pub nrows: i64,
        /// Original C++ type: `int64_t`.
        pub ncols: i64,
    }

    /// Stub for `tuple_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:50`.
    pub fn tuple_parse_handler_line_50<V: Clone + Default>(iter: usize) -> tuple_parse_handler<V> {
        tuple_parse_handler {
            flags: 1,
            begin_iter: iter,
            iter,
            tuples: Vec::new(),
        }
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:52`.
    pub fn handle_line_52<V: Clone + Default>(
        handler: &mut tuple_parse_handler<V>,
        row: i64,
        col: i64,
        value: V,
    ) {
        if handler.iter >= handler.tuples.len() {
            handler
                .tuples
                .resize(handler.iter + 1, (0, 0, V::default()));
        }
        handler.tuples[handler.iter] = (row, col, value);
        handler.iter += 1;
    }

    /// Stub for `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:57`.
    pub fn get_chunk_handler_line_57<V>(
        handler: &tuple_parse_handler<V>,
        offset_from_begin: i64,
    ) -> tuple_parse_handler<V>
    where
        V: Clone + Default,
    {
        let mut ret = handler.clone();
        ret.iter = handler.begin_iter + offset_from_begin.max(0) as usize;
        ret
    }

    /// Stub for `triplet_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:76`.
    pub fn triplet_parse_handler_line_76<V: Clone + Default>(
        rows: usize,
        cols: usize,
        values: usize,
    ) -> triplet_parse_handler<V> {
        triplet_parse_handler {
            flags: 1,
            begin_rows: rows,
            begin_cols: cols,
            begin_values: values,
            rows,
            cols,
            values,
            row_values: Vec::new(),
            col_values: Vec::new(),
            entry_values: Vec::new(),
        }
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:81`.
    pub fn handle_line_81<V>(handler: &mut triplet_parse_handler<V>, row: i64, col: i64, value: V)
    where
        V: Clone + Default,
    {
        if handler.rows >= handler.row_values.len() {
            handler.row_values.resize(handler.rows + 1, 0);
        }
        if handler.cols >= handler.col_values.len() {
            handler.col_values.resize(handler.cols + 1, 0);
        }
        if handler.values >= handler.entry_values.len() {
            handler
                .entry_values
                .resize(handler.values + 1, V::default());
        }
        handler.row_values[handler.rows] = row;
        handler.col_values[handler.cols] = col;
        handler.entry_values[handler.values] = value;
        handler.rows += 1;
        handler.cols += 1;
        handler.values += 1;
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:91`.
    pub fn handle_line_91<V: Clone + Default>(
        handler: &mut triplet_parse_handler<V>,
        row: i64,
        col: i64,
    ) {
        handle_line_81(handler, row, col, V::default());
    }

    /// Stub for `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:100`.
    pub fn get_chunk_handler_line_100<V>(
        handler: &triplet_parse_handler<V>,
        offset_from_begin: i64,
    ) -> triplet_parse_handler<V>
    where
        V: Clone + Default,
    {
        let mut ret = handler.clone();
        let offset = offset_from_begin.max(0) as usize;
        ret.rows = handler.begin_rows + offset;
        ret.cols = handler.begin_cols + offset;
        ret.values = handler.begin_values + offset;
        ret
    }

    /// Stub for `triplet_pattern_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:126`.
    pub fn triplet_pattern_parse_handler_line_126(
        rows: usize,
        cols: usize,
    ) -> triplet_pattern_parse_handler {
        triplet_pattern_parse_handler {
            flags: 1,
            begin_rows: rows,
            begin_cols: cols,
            rows,
            cols,
            row_values: Vec::new(),
            col_values: Vec::new(),
        }
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:130`.
    pub fn handle_line_130(handler: &mut triplet_pattern_parse_handler, row: i64, col: i64) {
        if handler.rows >= handler.row_values.len() {
            handler.row_values.resize(handler.rows + 1, 0);
        }
        if handler.cols >= handler.col_values.len() {
            handler.col_values.resize(handler.cols + 1, 0);
        }
        handler.row_values[handler.rows] = row;
        handler.col_values[handler.cols] = col;
        handler.rows += 1;
        handler.cols += 1;
    }

    /// Stub for `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:138`.
    pub fn get_chunk_handler_line_138(
        handler: &triplet_pattern_parse_handler,
        offset_from_begin: i64,
    ) -> triplet_pattern_parse_handler {
        let mut ret = handler.clone();
        let offset = offset_from_begin.max(0) as usize;
        ret.rows = handler.begin_rows + offset;
        ret.cols = handler.begin_cols + offset;
        ret
    }

    /// Stub for `triplet_calling_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:160`.
    pub fn triplet_calling_parse_handler_line_160<V>(
        rows: Vec<i64>,
        cols: Vec<i64>,
        values: Vec<V>,
        offset: i64,
    ) -> triplet_calling_parse_handler<V>
    where
        V: Clone + Default,
    {
        triplet_calling_parse_handler {
            flags: 1,
            offset,
            row_values: rows,
            col_values: cols,
            entry_values: values,
        }
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:165`.
    pub fn handle_line_165<V>(
        handler: &mut triplet_calling_parse_handler<V>,
        row: i64,
        col: i64,
        value: V,
    ) where
        V: Clone + Default,
    {
        let offset = handler.offset.max(0) as usize;
        if offset >= handler.row_values.len() {
            handler.row_values.resize(offset + 1, 0);
        }
        if offset >= handler.col_values.len() {
            handler.col_values.resize(offset + 1, 0);
        }
        if offset >= handler.entry_values.len() {
            handler.entry_values.resize(offset + 1, V::default());
        }
        handler.row_values[offset] = row;
        handler.col_values[offset] = col;
        handler.entry_values[offset] = value;
        handler.offset += 1;
    }

    /// Stub for `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:173`.
    pub fn get_chunk_handler_line_173<V>(
        handler: &triplet_calling_parse_handler<V>,
        offset_from_begin: i64,
    ) -> triplet_calling_parse_handler<V>
    where
        V: Clone + Default,
    {
        let mut ret = handler.clone();
        ret.offset = offset_from_begin;
        ret
    }

    /// Stub for `doublet_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:195`.
    pub fn doublet_parse_handler_line_195<V: Clone + Default>(
        index: usize,
        values: usize,
    ) -> doublet_parse_handler<V> {
        doublet_parse_handler {
            flags: 1,
            begin_index: index,
            begin_values: values,
            index,
            values,
            index_values: Vec::new(),
            entry_values: Vec::new(),
        }
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:199`.
    pub fn handle_line_199<V>(handler: &mut doublet_parse_handler<V>, row: i64, col: i64, value: V)
    where
        V: Clone + Default,
    {
        if handler.index >= handler.index_values.len() {
            handler.index_values.resize(handler.index + 1, 0);
        }
        if handler.values >= handler.entry_values.len() {
            handler
                .entry_values
                .resize(handler.values + 1, V::default());
        }
        handler.index_values[handler.index] = row.max(col);
        handler.entry_values[handler.values] = value;
        handler.index += 1;
        handler.values += 1;
    }

    /// Stub for `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:207`.
    pub fn get_chunk_handler_line_207<V>(
        handler: &doublet_parse_handler<V>,
        offset_from_begin: i64,
    ) -> doublet_parse_handler<V>
    where
        V: Clone + Default,
    {
        let mut ret = handler.clone();
        let offset = offset_from_begin.max(0) as usize;
        ret.index = handler.begin_index + offset;
        ret.values = handler.begin_values + offset;
        ret
    }

    /// Stub for `dense_2d_call_adding_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:229`.
    pub fn dense_2d_call_adding_parse_handler_line_229<V: dense_parse_value>(
        nrows: i64,
        ncols: i64,
    ) -> dense_2d_call_adding_parse_handler<V> {
        dense_2d_call_adding_parse_handler {
            flags: 3,
            nrows,
            ncols,
            values: vec![V::default(); (nrows * ncols).max(0) as usize],
        }
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:231`.
    pub fn handle_line_231<V: dense_parse_value>(
        handler: &mut dense_2d_call_adding_parse_handler<V>,
        row: i64,
        col: i64,
        value: V,
    ) {
        let offset = (row * handler.ncols + col) as usize;
        if offset >= handler.values.len() {
            handler.values.resize(offset + 1, V::default());
        }
        handler.values[offset].add_assign_value(&value);
    }

    /// Stub for `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:235`.
    pub fn get_chunk_handler_line_235<V: dense_parse_value>(
        handler: &dense_2d_call_adding_parse_handler<V>,
        _offset_from_begin: i64,
    ) -> dense_2d_call_adding_parse_handler<V> {
        handler.clone()
    }

    /// Stub for `dense_adding_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:253`.
    pub fn dense_adding_parse_handler_line_253<V: dense_parse_value>(
        values: Vec<V>,
        order: storage_order,
        nrows: i64,
        ncols: i64,
    ) -> dense_adding_parse_handler<V> {
        dense_adding_parse_handler {
            flags: 3,
            values,
            order,
            nrows,
            ncols,
        }
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:256`.
    pub fn handle_line_256<V: dense_parse_value>(
        handler: &mut dense_adding_parse_handler<V>,
        row: i64,
        col: i64,
        value: V,
    ) {
        let offset = if handler.order == storage_order::row_major {
            row * handler.ncols + col
        } else {
            col * handler.nrows + row
        } as usize;
        if offset >= handler.values.len() {
            handler.values.resize(offset + 1, V::default());
        }
        handler.values[offset].add_assign_value(&value);
    }

    /// Stub for `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:266`.
    pub fn get_chunk_handler_line_266<V: dense_parse_value>(
        handler: &dense_adding_parse_handler<V>,
        _offset_from_begin: i64,
    ) -> dense_adding_parse_handler<V> {
        handler.clone()
    }
}

pub mod read_body {
    use super::app_triplet::triplet_value_type;
    use super::chunking;
    use super::fast_matrix_market::{self, invalid_mm};
    use super::field_conv;
    use super::types::{
        field_type, format_type, generalize_coordinate_diagnonal_values_type, matrix_market_header,
        object_type, out_of_range_behavior, read_options, symmetry_type,
    };
    use super::Placeholder;
    use std::io::BufRead;

    /// Original struct `line_counts` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:14`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct line_counts {
        /// Original C++ type: `int64_t`.
        pub file_line: i64,
        /// Original C++ type: `int64_t`.
        pub element_num: i64,
    }

    /// Original class `pattern_parse_adapter` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:26`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct pattern_parse_adapter<V = Vec<f64>> {
        /// Original C++ type: `int`.
        pub flags: i64,
        /// Original C++ type: `FWD_HANDLER`.
        pub handler: Vec<(i64, i64, V)>,
        /// Original C++ type: `typename FWD_HANDLER::value_type`.
        pub fwd_value: V,
    }

    /// Original class `complex_parse_adapter` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:60`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct complex_parse_adapter<V = Vec<f64>> {
        /// Original C++ type: `int`.
        pub flags: i64,
        /// Original C++ type: `COMPLEX_HANDLER`.
        pub handler: Vec<(i64, i64, V)>,
    }

    /// Stub for `pattern_parse_adapter` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:32`.
    pub fn pattern_parse_adapter_line_32<V: Clone + Default>(
        handler: Vec<(i64, i64, V)>,
        fwd_value: V,
    ) -> pattern_parse_adapter<V> {
        pattern_parse_adapter {
            flags: 1,
            handler,
            fwd_value,
        }
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:35`.
    pub fn handle_line_35<V: Clone + Default>(
        adapter: &mut pattern_parse_adapter<V>,
        row: i64,
        col: i64,
    ) {
        adapter.handler.push((row, col, adapter.fwd_value.clone()));
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:39`.
    pub fn handle_line_39<V: Clone + Default>(
        adapter: &mut pattern_parse_adapter<V>,
        row: i64,
        col: i64,
        val: V,
    ) {
        adapter.handler.push((row, col, val));
    }

    /// Stub for `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:43`.
    pub fn get_chunk_handler_line_43<V>(
        adapter: &pattern_parse_adapter<V>,
        offset_from_start: i64,
    ) -> pattern_parse_adapter<V>
    where
        V: Clone + Default,
    {
        let mut ret = adapter.clone();
        let offset = offset_from_start.max(0) as usize;
        ret.handler = adapter.handler.iter().skip(offset).cloned().collect();
        ret
    }

    /// Stub for `complex_parse_adapter` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:67`.
    pub fn complex_parse_adapter_line_67<V: triplet_value_type>(
        handler: Vec<(i64, i64, V)>,
    ) -> complex_parse_adapter<V> {
        complex_parse_adapter { flags: 1, handler }
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:69`.
    pub fn handle_line_69<V: triplet_value_type>(
        adapter: &mut complex_parse_adapter<V>,
        row: i64,
        col: i64,
    ) {
        adapter.handler.push((row, col, V::pattern_value()));
    }

    /// Stub for `handle` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:73`.
    pub fn handle_line_73<V: triplet_value_type>(
        adapter: &mut complex_parse_adapter<V>,
        row: i64,
        col: i64,
        real: f64,
    ) {
        let value = V::complex_value(real, 0.0).unwrap_or_else(|_| V::real_value(real));
        adapter.handler.push((row, col, value));
    }

    /// Stub for `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:77`.
    pub fn get_chunk_handler_line_77<V>(
        adapter: &complex_parse_adapter<V>,
        offset_from_start: i64,
    ) -> complex_parse_adapter<V>
    where
        V: triplet_value_type,
    {
        let mut ret = adapter.clone();
        let offset = offset_from_start.max(0) as usize;
        ret.handler = adapter.handler.iter().skip(offset).cloned().collect();
        ret
    }

    /// Stub for `limit_parallelism_for_value_type` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:94`.
    pub fn limit_parallelism_for_value_type_line_94(_parallelism_selected: bool) -> bool {
        false
    }

    /// Stub for `limit_parallelism_for_value_type` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:99`.
    pub fn limit_parallelism_for_value_type_line_99(parallelism_selected: bool) -> bool {
        parallelism_selected
    }

    /// Stub for `get_symmetric_value` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:108`.
    pub fn get_symmetric_value_line_108<V: triplet_value_type>(
        value: &V,
        symmetry: symmetry_type,
    ) -> Result<V, fast_matrix_market::invalid_argument> {
        Ok(value.symmetric_value(symmetry))
    }

    /// Stub for `generalize_symmetry_coordinate` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:127`.
    pub fn generalize_symmetry_coordinate_line_127<V: triplet_value_type>(
        header: &matrix_market_header,
        _options: &read_options,
        row: i64,
        col: i64,
        value: &V,
    ) -> Result<Vec<(i64, i64, V)>, fast_matrix_market::invalid_argument> {
        let mut ret = Vec::new();
        if col != row {
            if header.symmetry != symmetry_type::general {
                ret.push((
                    col,
                    row,
                    get_symmetric_value_line_108(value, header.symmetry)?,
                ));
            }
        }
        Ok(ret)
    }

    /// Stub for `generalize_symmetry_array` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:165`.
    pub fn generalize_symmetry_array_line_165<V: triplet_value_type>(
        header: &matrix_market_header,
        row: i64,
        col: i64,
        value: &V,
    ) -> Result<Vec<(i64, i64, V)>, fast_matrix_market::invalid_argument> {
        if header.symmetry == symmetry_type::general {
            Ok(Vec::new())
        } else {
            Ok(vec![(
                col,
                row,
                get_symmetric_value_line_108(value, header.symmetry)?,
            )])
        }
    }

    /// Stub for `read_real_or_complex` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:193`.
    pub fn read_real_or_complex_line_193<V: triplet_value_type>(
        fields: &[&str],
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<V, invalid_mm> {
        V::check_header_field(header.field)?;
        let parse_float = |field: &str| -> Result<f64, invalid_mm> {
            let (next, value) =
                field_conv::read_float_fallback_line_196(field, 0).map_err(|_| {
                    fast_matrix_market::invalid_mm_line_54(
                        "Invalid floating-point value.".to_string(),
                    )
                })?;
            if options.float_out_of_range_behavior == out_of_range_behavior::ThrowOutOfRange
                && value.is_infinite()
            {
                let token = &field.as_bytes()[..next];
                let start = if !token.is_empty() && (token[0] == b'+' || token[0] == b'-') {
                    1
                } else {
                    0
                };
                let explicit_infinity = (token.len() == start + "inf".len()
                    && token[start].to_ascii_lowercase() == b'i'
                    && token[start + 1].to_ascii_lowercase() == b'n'
                    && token[start + 2].to_ascii_lowercase() == b'f')
                    || (token.len() == start + "infinity".len()
                        && token[start].to_ascii_lowercase() == b'i'
                        && token[start + 1].to_ascii_lowercase() == b'n'
                        && token[start + 2].to_ascii_lowercase() == b'f'
                        && token[start + 3].to_ascii_lowercase() == b'i'
                        && token[start + 4].to_ascii_lowercase() == b'n'
                        && token[start + 5].to_ascii_lowercase() == b'i'
                        && token[start + 6].to_ascii_lowercase() == b't'
                        && token[start + 7].to_ascii_lowercase() == b'y');
                if !explicit_infinity {
                    return Err(fast_matrix_market::invalid_mm_line_54(
                        "Floating-point value out of range.".to_string(),
                    ));
                }
            }
            Ok(value)
        };
        if header.field == field_type::pattern {
            return Ok(V::pattern_value());
        }
        if header.field == field_type::complex {
            if fields.len() < 2 {
                return Err(fast_matrix_market::invalid_mm_line_54(
                    "Invalid floating-point value.".to_string(),
                ));
            }
            let real = parse_float(fields[0])?;
            let imaginary = parse_float(fields[1])?;
            V::complex_value(real, imaginary)
        } else {
            if fields.is_empty() {
                return Err(fast_matrix_market::invalid_mm_line_54(
                    "Invalid floating-point value.".to_string(),
                ));
            }
            let value = parse_float(fields[0])?;
            Ok(V::real_value(value))
        }
    }

    /// Stub for `read_chunk_matrix_coordinate` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:213`.
    pub fn read_chunk_matrix_coordinate_line_213<V: triplet_value_type>(
        chunk: &str,
        header: &matrix_market_header,
        mut line: line_counts,
        options: &read_options,
    ) -> Result<(line_counts, Vec<(i64, i64, V)>), invalid_mm> {
        V::check_header_field(header.field)?;
        let mut entries = Vec::new();
        for raw_line in chunk.split_inclusive('\n') {
            let trimmed = raw_line.trim_matches([' ', '\t', '\r', '\n']);
            if trimmed.is_empty() {
                line.file_line += raw_line.bytes().filter(|&c| c == b'\n').count() as i64;
                continue;
            }
            if line.element_num >= header.nnz {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Too many lines in file (file too long)".to_string(),
                    line.file_line + 1,
                ));
            }
            let mut fields = trimmed.split_whitespace();
            let row_field = fields.next().ok_or_else(|| {
                fast_matrix_market::invalid_mm_line_55(
                    "Invalid integer value.".to_string(),
                    line.file_line + 1,
                )
            })?;
            let col_field = fields.next().ok_or_else(|| {
                fast_matrix_market::invalid_mm_line_55(
                    "Invalid integer value.".to_string(),
                    line.file_line + 1,
                )
            })?;
            let (row_end, row_one) = field_conv::read_int_line_140(row_field, 0).map_err(|_| {
                fast_matrix_market::invalid_mm_line_55(
                    "Invalid integer value.".to_string(),
                    line.file_line + 1,
                )
            })?;
            if row_end != row_field.len() {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Invalid integer value.".to_string(),
                    line.file_line + 1,
                ));
            }
            let (col_end, col_one) = field_conv::read_int_line_140(col_field, 0).map_err(|_| {
                fast_matrix_market::invalid_mm_line_55(
                    "Invalid integer value.".to_string(),
                    line.file_line + 1,
                )
            })?;
            if col_end != col_field.len() {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Invalid integer value.".to_string(),
                    line.file_line + 1,
                ));
            }
            if row_one <= 0 || row_one > header.nrows {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Row index out of bounds".to_string(),
                    line.file_line + 1,
                ));
            }
            if col_one <= 0 || col_one > header.ncols {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Column index out of bounds".to_string(),
                    line.file_line + 1,
                ));
            }
            let row = row_one - 1;
            let col = col_one - 1;
            let value = if header.field == field_type::pattern {
                read_real_or_complex_line_193::<V>(&[], header, options)
            } else if header.field == field_type::complex {
                let real_field = fields.next().unwrap_or("");
                let imaginary_field = fields.next().unwrap_or("");
                read_real_or_complex_line_193::<V>(&[real_field, imaginary_field], header, options)
            } else {
                let value_field = fields.next().unwrap_or("");
                read_real_or_complex_line_193::<V>(&[value_field], header, options)
            }
            .map_err(|err| fast_matrix_market::invalid_mm_line_55(err.msg, line.file_line + 1))?;
            if header.symmetry != symmetry_type::general
                && options.generalize_symmetry
                && col != row
            {
                entries.push((col, row, value.symmetric_value(header.symmetry)));
            }
            entries.push((row, col, value));
            line.file_line += 1;
            line.element_num += 1;
        }
        Ok((line, entries))
    }

    /// Stub for `read_chunk_vector_coordinate` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:281`.
    pub fn read_chunk_vector_coordinate_line_281<V: triplet_value_type>(
        chunk: &str,
        header: &matrix_market_header,
        mut line: line_counts,
        options: &read_options,
    ) -> Result<(line_counts, Vec<(i64, i64, V)>), invalid_mm> {
        V::check_header_field(header.field)?;
        let mut entries = Vec::new();
        for raw_line in chunk.split_inclusive('\n') {
            let trimmed = raw_line.trim_matches([' ', '\t', '\r', '\n']);
            if trimmed.is_empty() {
                line.file_line += raw_line.bytes().filter(|&c| c == b'\n').count() as i64;
                continue;
            }
            if line.element_num >= header.nnz {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Too many lines in file (file too long)".to_string(),
                    line.file_line + 1,
                ));
            }
            let mut fields = trimmed.split_whitespace();
            let row_field = fields.next().ok_or_else(|| {
                fast_matrix_market::invalid_mm_line_55(
                    "Invalid integer value.".to_string(),
                    line.file_line + 1,
                )
            })?;
            let (row_end, row_one) = field_conv::read_int_line_140(row_field, 0).map_err(|_| {
                fast_matrix_market::invalid_mm_line_55(
                    "Invalid integer value.".to_string(),
                    line.file_line + 1,
                )
            })?;
            if row_end != row_field.len() {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Invalid integer value.".to_string(),
                    line.file_line + 1,
                ));
            }
            if row_one <= 0 || row_one > header.vector_length {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Vector index out of bounds".to_string(),
                    line.file_line + 1,
                ));
            }
            let value = if header.field == field_type::pattern {
                read_real_or_complex_line_193::<V>(&[], header, options)
            } else if header.field == field_type::complex {
                let real_field = fields.next().unwrap_or("");
                let imaginary_field = fields.next().unwrap_or("");
                read_real_or_complex_line_193::<V>(&[real_field, imaginary_field], header, options)
            } else {
                let value_field = fields.next().unwrap_or("");
                read_real_or_complex_line_193::<V>(&[value_field], header, options)
            }
            .map_err(|err| fast_matrix_market::invalid_mm_line_55(err.msg, line.file_line + 1))?;
            entries.push((row_one - 1, 0, value));
            line.file_line += 1;
            line.element_num += 1;
        }
        Ok((line, entries))
    }

    /// Stub for `read_chunk_array` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:332`.
    pub fn read_chunk_array_line_332<V: triplet_value_type>(
        chunk: &str,
        header: &matrix_market_header,
        mut line: line_counts,
        options: &read_options,
        row: &mut i64,
        col: &mut i64,
    ) -> Result<(line_counts, Vec<(i64, i64, V)>), invalid_mm> {
        V::check_header_field(header.field)?;
        let mut entries = Vec::new();
        if header.symmetry == symmetry_type::skew_symmetric
            && *row == 0
            && *col == 0
            && header.nrows > 0
        {
            *row = 1;
        }
        for raw_line in chunk.split_inclusive('\n') {
            let trimmed = raw_line.trim_matches([' ', '\t', '\r', '\n']);
            if trimmed.is_empty() {
                line.file_line += raw_line.bytes().filter(|&c| c == b'\n').count() as i64;
                continue;
            }
            if *col >= header.ncols {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Too many values in array (file too long)".to_string(),
                    line.file_line + 1,
                ));
            }
            let mut fields = trimmed.split_whitespace();
            let value = if header.field == field_type::pattern {
                read_real_or_complex_line_193::<V>(&[], header, options)
            } else if header.field == field_type::complex {
                let real_field = fields.next().unwrap_or("");
                let imaginary_field = fields.next().unwrap_or("");
                read_real_or_complex_line_193::<V>(&[real_field, imaginary_field], header, options)
            } else {
                let value_field = fields.next().unwrap_or("");
                read_real_or_complex_line_193::<V>(&[value_field], header, options)
            }
            .map_err(|err| fast_matrix_market::invalid_mm_line_55(err.msg, line.file_line + 1))?;
            entries.push((*row, *col, value.clone()));
            if *row != *col && options.generalize_symmetry {
                if header.symmetry != symmetry_type::general {
                    entries.push((*col, *row, value.symmetric_value(header.symmetry)));
                }
            }
            *row += 1;
            if *row == header.nrows {
                *col += 1;
                if header.symmetry == symmetry_type::general {
                    *row = 0;
                } else {
                    *row = *col;
                    if header.symmetry == symmetry_type::skew_symmetric && *row < header.nrows - 1 {
                        *row += 1;
                    }
                }
            }
            line.file_line += 1;
            line.element_num += 1;
        }
        Ok((line, entries))
    }

    /// Stub for `read_coordinate_body_sequential` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:412`.
    pub fn read_coordinate_body_sequential_line_412<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<(line_counts, Vec<(i64, i64, V)>), invalid_mm> {
        let mut lc = line_counts {
            file_line: header.header_line_count,
            element_num: 0,
        };
        let mut entries = Vec::new();
        loop {
            let chunk = chunking::get_next_chunk_line_51(instream, options)
                .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
            if chunk.is_empty() {
                break;
            }
            let (next_lc, mut next_entries) = if header.object == object_type::matrix {
                read_chunk_matrix_coordinate_line_213::<V>(&chunk, header, lc, options)?
            } else {
                read_chunk_vector_coordinate_line_281::<V>(&chunk, header, lc, options)?
            };
            lc = next_lc;
            entries.append(&mut next_entries);
        }
        Ok((lc, entries))
    }

    /// Stub for `read_array_body_sequential` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:436`.
    pub fn read_array_body_sequential_line_436<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<(line_counts, Vec<(i64, i64, V)>), invalid_mm> {
        let mut lc = line_counts {
            file_line: header.header_line_count,
            element_num: 0,
        };
        let mut row = 0i64;
        let mut col = 0i64;
        let mut entries = Vec::new();
        loop {
            let chunk = chunking::get_next_chunk_line_51(instream, options)
                .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
            if chunk.is_empty() {
                break;
            }
            let (next_lc, mut next_entries) =
                read_chunk_array_line_332::<V>(&chunk, header, lc, options, &mut row, &mut col)?;
            lc = next_lc;
            entries.append(&mut next_entries);
        }
        Ok((lc, entries))
    }

    /// Stub for `read_matrix_market_body_no_adapters` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:459`.
    pub fn read_matrix_market_body_no_adapters_line_459<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<Vec<(i64, i64, V)>, invalid_mm> {
        V::check_header_field(header.field)?;
        if header.object == object_type::vector && header.symmetry != symmetry_type::general {
            return Err(fast_matrix_market::invalid_mm_line_54(
                "Vectors cannot have symmetry.".to_string(),
            ));
        }
        if header.format == format_type::array && header.field == field_type::pattern {
            return Err(fast_matrix_market::invalid_mm_line_54(
                "Array matrices may not be pattern.".to_string(),
            ));
        }

        let (lc, entries) = if header.format == format_type::coordinate {
            read_coordinate_body_sequential_line_412::<V>(instream, header, options)?
        } else {
            read_array_body_sequential_line_436::<V>(instream, header, options)?
        };
        if lc.element_num < header.nnz
            && !(header.symmetry != symmetry_type::general && header.format == format_type::array)
        {
            return Err(fast_matrix_market::invalid_mm_line_54(format!(
                "Truncated file. Expected another {} lines.",
                header.nnz - lc.element_num
            )));
        }
        Ok(entries)
    }

    /// Stub for `read_matrix_market_body_no_pattern` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:525`.
    pub fn read_matrix_market_body_no_pattern_line_525<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<Vec<(i64, i64, V)>, invalid_mm> {
        read_matrix_market_body_no_adapters_line_459::<V>(instream, header, options)
    }

    /// Stub for `read_matrix_market_body_no_pattern` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:543`.
    pub fn read_matrix_market_body_no_pattern_line_543<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<Vec<(i64, i64, V)>, invalid_mm> {
        if header.field == field_type::complex {
            Err(fast_matrix_market::invalid_mm_line_54(
                "Matrix Market file has complex fields but passed data structure cannot handle complex values."
                    .to_string(),
            ))
        } else {
            read_matrix_market_body_no_adapters_line_459::<V>(instream, header, options)
        }
    }

    /// Stub for `read_matrix_market_body` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:562`.
    pub fn read_matrix_market_body_line_562<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<Vec<(i64, i64, V)>, invalid_mm> {
        read_matrix_market_body_no_adapters_line_459::<V>(instream, header, options)
    }
}

pub mod read_body_threads {
    use super::app_triplet::triplet_value_type;
    use super::fast_matrix_market;
    use super::read_body::{self, line_counts};
    use super::types::{matrix_market_header, read_options};
    use super::Placeholder;
    use std::io::BufRead;

    /// Original struct `line_count_result_s` at `fast_matrix_market/include/fast_matrix_market/read_body_threads.hpp:15`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct line_count_result_s {
        /// Original C++ type: `std::string`.
        pub chunk: String,
        /// Original C++ type: `line_counts`.
        pub counts: line_counts,
    }

    /// Stub for `line_count_result_s` at `fast_matrix_market/include/fast_matrix_market/read_body_threads.hpp:19`.
    pub fn line_count_result_s_line_19(chunk: String, counts: line_counts) -> line_count_result_s {
        line_count_result_s { chunk, counts }
    }

    /// Stub for `count_chunk_lines` at `fast_matrix_market/include/fast_matrix_market/read_body_threads.hpp:24`.
    pub fn count_chunk_lines_line_24(chunk: String) -> line_count_result_s {
        let (file_line, element_num) = super::chunking::count_lines_line_66(&chunk);
        line_count_result_s_line_19(
            chunk,
            line_counts {
                file_line,
                element_num,
            },
        )
    }

    /// Stub for `read_body_threads` at `fast_matrix_market/include/fast_matrix_market/read_body_threads.hpp:33`.
    pub fn read_body_threads_line_33<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<(line_counts, Vec<(i64, i64, V)>), fast_matrix_market::invalid_mm> {
        V::check_header_field(header.field)?;
        let mut lc = line_counts {
            file_line: header.header_line_count,
            element_num: 0,
        };
        let mut row = 0i64;
        let mut col = 0i64;
        let mut entries = Vec::new();
        let mut line_count_results = Vec::new();
        let mut reuse_pool = Vec::new();
        let generalizing_symmetry_factor = if header.symmetry
            != super::types::symmetry_type::general
            && options.generalize_symmetry
        {
            2
        } else {
            1
        };

        loop {
            let chunk = super::chunking::get_next_chunk_line_51(instream, options)
                .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
            if chunk.is_empty() {
                break;
            }
            let counted = count_chunk_lines_line_24(chunk);
            reuse_pool.push(counted.chunk.clone());
            line_count_results.push(counted);
            let (next_lc, mut next_entries) =
                if header.format == super::types::format_type::coordinate {
                    if header.object == super::types::object_type::matrix {
                        read_body::read_chunk_matrix_coordinate_line_213::<V>(
                            &line_count_results.last().unwrap().chunk,
                            header,
                            lc,
                            options,
                        )?
                    } else {
                        read_body::read_chunk_vector_coordinate_line_281::<V>(
                            &line_count_results.last().unwrap().chunk,
                            header,
                            lc,
                            options,
                        )?
                    }
                } else {
                    read_body::read_chunk_array_line_332::<V>(
                        &line_count_results.last().unwrap().chunk,
                        header,
                        lc,
                        options,
                        &mut row,
                        &mut col,
                    )?
                };
            lc = next_lc;
            if generalizing_symmetry_factor > 1 {
                entries.reserve(next_entries.len());
            }
            entries.append(&mut next_entries);
        }
        line_count_results.clear();
        reuse_pool.clear();
        Ok((lc, entries))
    }
}

pub mod thirdparty_task_thread_pool {
    use super::Placeholder;

    /// Original class `task_thread_pool` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:130`.
    pub struct task_thread_pool {
        /// Original C++ type: `std::vector<std::thread>`.
        pub threads: Vec<Placeholder>,
        /// Original C++ type: `std::mutex`.
        pub thread_mutex: Placeholder,
        /// Original C++ type: `std::queue<std::packaged_task<void()>>`.
        pub tasks: Placeholder,
        pub queued_tasks: Vec<Box<dyn FnMut() + Send>>,
        /// Original C++ type: `std::mutex`.
        pub task_mutex: Placeholder,
        /// Original C++ type: `std::condition_variable`.
        pub task_cv: Placeholder,
        /// Original C++ type: `std::condition_variable`.
        pub task_finished_cv: Placeholder,
        /// Original C++ type: `bool`.
        pub pool_running: bool,
        /// Original C++ type: `bool`.
        pub pool_paused: bool,
        /// Original C++ type: `bool`.
        pub notify_task_finish: bool,
        /// Original C++ type: `int`.
        pub num_inflight_tasks: i64,
    }

    /// Stub for `task_thread_pool` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:137`.
    pub fn task_thread_pool_line_137(num_threads: usize) -> task_thread_pool {
        let mut pool = task_thread_pool {
            threads: Vec::new(),
            thread_mutex: Placeholder,
            tasks: Placeholder,
            queued_tasks: Vec::new(),
            task_mutex: Placeholder,
            task_cv: Placeholder,
            task_finished_cv: Placeholder,
            pool_running: true,
            pool_paused: false,
            notify_task_finish: false,
            num_inflight_tasks: 0,
        };
        start_threads_line_345(&mut pool, num_threads);
        pool
    }

    /// Stub for `~task_thread_pool` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:149`.
    pub fn task_thread_pool_line_149(pool: &mut task_thread_pool) {
        unpause_line_218(pool);
        wait_for_tasks_line_291(pool);
        stop_all_threads_line_356(pool);
    }

    /// Stub for `clear_task_queue` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:160`.
    pub fn clear_task_queue_line_160(pool: &mut task_thread_pool) {
        pool.queued_tasks.clear();
    }

    /// Stub for `get_num_queued_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:170`.
    pub fn get_num_queued_tasks_line_170(pool: &task_thread_pool) -> usize {
        pool.queued_tasks.len()
    }

    /// Stub for `get_num_running_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:180`.
    pub fn get_num_running_tasks_line_180(pool: &task_thread_pool) -> usize {
        pool.num_inflight_tasks.max(0) as usize
    }

    /// Stub for `get_num_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:190`.
    pub fn get_num_tasks_line_190(pool: &task_thread_pool) -> usize {
        pool.queued_tasks.len() + pool.num_inflight_tasks.max(0) as usize
    }

    /// Stub for `get_num_threads` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:200`.
    pub fn get_num_threads_line_200(pool: &task_thread_pool) -> usize {
        pool.threads.len()
    }

    /// Stub for `pause` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:210`.
    pub fn pause_line_210(pool: &mut task_thread_pool) {
        pool.pool_paused = true;
    }

    /// Stub for `unpause` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:218`.
    pub fn unpause_line_218(pool: &mut task_thread_pool) {
        pool.pool_paused = false;
    }

    /// Stub for `is_paused` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:229`.
    pub fn is_paused_line_229(pool: &task_thread_pool) -> bool {
        pool.pool_paused
    }

    /// Stub for `std::future<R> submit` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:248`.
    pub fn std_future_r_submit_line_248<F, R>(
        pool: &mut task_thread_pool,
        mut func: F,
    ) -> std::sync::mpsc::Receiver<R>
    where
        F: FnMut() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        submit_detach_line_260(pool, move || {
            let _ = tx.send(func());
        });
        rx
    }

    /// Stub for `submit_detach` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:260`.
    pub fn submit_detach_line_260<F>(pool: &mut task_thread_pool, func: F)
    where
        F: FnMut() + Send + 'static,
    {
        pool.queued_tasks.push(Box::new(func));
    }

    /// Stub for `submit_detach` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:272`.
    pub fn submit_detach_line_272<F>(pool: &mut task_thread_pool, func: F)
    where
        F: FnMut() + Send + 'static,
    {
        submit_detach_line_260(pool, func);
    }

    /// Stub for `wait_for_queued_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:281`.
    pub fn wait_for_queued_tasks_line_281(pool: &mut task_thread_pool) {
        worker_main_line_303(pool);
    }

    /// Stub for `wait_for_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:291`.
    pub fn wait_for_tasks_line_291(pool: &mut task_thread_pool) {
        worker_main_line_303(pool);
    }

    /// Stub for `worker_main` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:303`.
    pub fn worker_main_line_303(pool: &mut task_thread_pool) {
        if pool.pool_paused || !pool.pool_running {
            return;
        }
        while let Some(mut task) = pool.queued_tasks.pop() {
            pool.num_inflight_tasks += 1;
            task();
            pool.num_inflight_tasks -= 1;
        }
    }

    /// Stub for `start_threads` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:345`.
    pub fn start_threads_line_345(pool: &mut task_thread_pool, num_threads: usize) {
        let count = if num_threads < 1 { 1 } else { num_threads };
        pool.threads = vec![Placeholder; count];
    }

    /// Stub for `stop_all_threads` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:356`.
    pub fn stop_all_threads_line_356(pool: &mut task_thread_pool) {
        pool.pool_running = false;
        pool.threads.clear();
    }
}

pub mod types {
    use super::Placeholder;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum object_type {
        matrix,
        vector,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum format_type {
        array,
        coordinate,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum field_type {
        real,
        double_,
        complex,
        integer,
        pattern,
        unsigned_integer,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum symmetry_type {
        general,
        symmetric,
        skew_symmetric,
        hermitian,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum storage_order {
        row_major = 1,
        col_major = 2,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum out_of_range_behavior {
        BestMatch = 1,
        ThrowOutOfRange = 2,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum generalize_coordinate_diagnonal_values_type {
        ExtraZeroElement,
        DuplicateElement,
    }

    /// Original struct `matrix_market_header` at `fast_matrix_market/include/fast_matrix_market/types.hpp:47`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct matrix_market_header {
        /// Original C++ type: `object_type`.
        pub object: object_type,
        /// Original C++ type: `format_type`.
        pub format: format_type,
        /// Original C++ type: `field_type`.
        pub field: field_type,
        /// Original C++ type: `symmetry_type`.
        pub symmetry: symmetry_type,
        /// Original C++ type: `int64_t`.
        pub nrows: i64,
        /// Original C++ type: `int64_t`.
        pub ncols: i64,
        /// Original C++ type: `int64_t`.
        pub vector_length: i64,
        /// Original C++ type: `int64_t`.
        pub nnz: i64,
        /// Original C++ type: `std::string`.
        pub comment: String,
        /// Original C++ type: `int64_t`.
        pub header_line_count: i64,
    }

    /// Original struct `read_options` at `fast_matrix_market/include/fast_matrix_market/types.hpp:77`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct read_options {
        /// Original C++ type: `int64_t`.
        pub chunk_size_bytes: i64,
        /// Original C++ type: `bool`.
        pub generalize_symmetry: bool,
        /// Original C++ type: `bool`.
        pub generalize_symmetry_app: bool,
        /// Original C++ type: `enum {ExtraZeroElement, DuplicateElement}`.
        pub generalize_coordinate_diagnonal_values: generalize_coordinate_diagnonal_values_type,
        /// Original C++ type: `bool`.
        pub parallel_ok: bool,
        /// Original C++ type: `int`.
        pub num_threads: i64,
        /// Original C++ type: `out_of_range_behavior`.
        pub float_out_of_range_behavior: out_of_range_behavior,
    }

    /// Original struct `write_options` at `fast_matrix_market/include/fast_matrix_market/types.hpp:132`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct write_options {
        /// Original C++ type: `int64_t`.
        pub chunk_size_values: i64,
        /// Original C++ type: `bool`.
        pub parallel_ok: bool,
        /// Original C++ type: `int`.
        pub num_threads: i64,
        /// Original C++ type: `int`.
        pub precision: i64,
        /// Original C++ type: `bool`.
        pub always_comment: bool,
        /// Original C++ type: `bool`.
        pub fill_header_field_type: bool,
    }

    /// Original struct `is_complex` at `fast_matrix_market/include/fast_matrix_market/types.hpp:169`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct is_complex {}

    /// Original struct `is_complex<std::complex<T>>` at `fast_matrix_market/include/fast_matrix_market/types.hpp:170`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct is_complex_std_complex_T_line_170 {}

    /// Original struct `can_read_complex` at `fast_matrix_market/include/fast_matrix_market/types.hpp:172`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct can_read_complex {}

    /// Stub for `matrix_market_header` at `fast_matrix_market/include/fast_matrix_market/types.hpp:48`.
    pub fn matrix_market_header_line_48() -> matrix_market_header {
        matrix_market_header {
            object: object_type::matrix,
            format: format_type::coordinate,
            field: field_type::real,
            symmetry: symmetry_type::general,
            nrows: 0,
            ncols: 0,
            vector_length: 0,
            nnz: 0,
            comment: String::new(),
            header_line_count: 1,
        }
    }

    /// Stub for `matrix_market_header` at `fast_matrix_market/include/fast_matrix_market/types.hpp:49`.
    pub fn matrix_market_header_line_49(vector_length: i64) -> matrix_market_header {
        let mut header = matrix_market_header_line_48();
        header.object = object_type::vector;
        header.vector_length = vector_length;
        header
    }

    /// Stub for `matrix_market_header` at `fast_matrix_market/include/fast_matrix_market/types.hpp:50`.
    pub fn matrix_market_header_line_50(nrows: i64, ncols: i64) -> matrix_market_header {
        let mut header = matrix_market_header_line_48();
        header.nrows = nrows;
        header.ncols = ncols;
        header
    }
}

pub mod write_body {
    use super::fast_matrix_market::pattern_placeholder_type;
    use super::types::{field_type, write_options};
    use super::Placeholder;
    use std::io::Write;

    /// Stub for `get_field_type` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:16`.
    pub fn get_field_type_line_16<T>(_type: *const T) -> field_type {
        field_type::integer
    }

    /// Stub for `get_field_type` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:24`.
    pub fn get_field_type_line_24<T>(_type: *const T) -> field_type {
        field_type::real
    }

    /// Stub for `get_field_type` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:32`.
    pub fn get_field_type_line_32<T>(_type: *const T) -> field_type {
        field_type::complex
    }

    /// Stub for `get_field_type` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:40`.
    pub fn get_field_type_line_40(_type: *const pattern_placeholder_type) -> field_type {
        field_type::pattern
    }

    /// Stub for `write_body_sequential` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:50`.
    pub fn write_body_sequential_line_50<W: Write>(
        os: &mut W,
        chunks: &[String],
        _options: &write_options,
    ) -> std::io::Result<()> {
        for chunk in chunks {
            os.write_all(chunk.as_bytes())?;
        }
        Ok(())
    }

    /// Stub for `write_body` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:66`.
    pub fn write_body_line_66<W: Write>(
        os: &mut W,
        chunks: &[String],
        options: &write_options,
    ) -> std::io::Result<()> {
        write_body_sequential_line_50(os, chunks, options)
    }
}

pub mod write_body_threads {
    use super::types::write_options;
    use super::Placeholder;
    use std::io::Write;

    /// Stub for `write_body_threads` at `fast_matrix_market/include/fast_matrix_market/write_body_threads.hpp:20`.
    pub fn write_body_threads_line_20<W: Write>(
        os: &mut W,
        chunks: &[String],
        _options: &write_options,
    ) -> std::io::Result<()> {
        for chunk in chunks {
            os.write_all(chunk.as_bytes())?;
        }
        Ok(())
    }
}
