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
//! Each generated item records the original C++ header location where useful.
//! Out-of-scope integration bindings are omitted from this crate.

/// Placeholder for C++ template parameters, iterators, pointers, and dependent types.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Placeholder;

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

    /// Translation of `read_matrix_market_array` at `fast_matrix_market/include/fast_matrix_market/app/array.hpp:46`.
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

    /// Translation of `read_matrix_market_array` at `fast_matrix_market/include/fast_matrix_market/app/array.hpp:66`.
    pub fn read_matrix_market_array_line_66<V: triplet_value_type + Default>(
        instream: &mut impl BufRead,
        order: storage_order,
        options: &read_options,
    ) -> Result<(i64, i64, Vec<V>), super::fast_matrix_market::invalid_mm> {
        let mut header = super::types::matrix_market_header_line_48();
        let values = read_matrix_market_array_line_46(instream, &mut header, order, options)?;
        Ok((header.nrows, header.ncols, values))
    }

    /// Translation of `read_matrix_market_array` at `fast_matrix_market/include/fast_matrix_market/app/array.hpp:82`.
    pub fn read_matrix_market_array_line_82<V: triplet_value_type + Default>(
        instream: &mut impl BufRead,
        order: storage_order,
        options: &read_options,
    ) -> Result<Vec<V>, super::fast_matrix_market::invalid_mm> {
        let mut header = super::types::matrix_market_header_line_48();
        read_matrix_market_array_line_46(instream, &mut header, order, options)
    }

    /// Translation of `write_matrix_market_array` at `fast_matrix_market/include/fast_matrix_market/app/array.hpp:94`.
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

    /// Translation of `read_matrix_market_doublet` at `fast_matrix_market/include/fast_matrix_market/app/doublet.hpp:47`.
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

    /// Translation of `read_matrix_market_doublet` at `fast_matrix_market/include/fast_matrix_market/app/doublet.hpp:66`.
    pub fn read_matrix_market_doublet_line_66<V: triplet_value_type>(
        instream: &mut impl BufRead,
        options: &read_options,
    ) -> Result<(i64, Vec<i64>, Vec<V>), super::fast_matrix_market::invalid_mm> {
        let mut header = super::types::matrix_market_header_line_48();
        let (indices, values) =
            read_matrix_market_doublet_line_47::<V>(instream, &mut header, options)?;
        Ok((header.vector_length, indices, values))
    }

    /// Translation of `write_matrix_market_doublet` at `fast_matrix_market/include/fast_matrix_market/app/doublet.hpp:79`.
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

    /// Translation of `coo_independent_generator_formatter` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:17`.
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

    /// Translation of `has_next` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:24`.
    pub fn has_next_line_24<V>(formatter: &coo_independent_generator_formatter<V>) -> bool {
        formatter.next_chunk_offset < formatter.nnz
    }

    /// Translation of `chunk` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:30`.
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

    /// Translation of `operator()` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:34`.
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

    /// Translation of `next_chunk` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:54`.
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

    /// Translation of `write_matrix_market_generated_triplet` at `fast_matrix_market/include/fast_matrix_market/app/generator.hpp:81`.
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

pub mod app_triplet {
    use super::chunking;
    use super::fast_matrix_market;
    use super::field_conv;
    use super::formatters;
    use super::header;
    use super::read_body::{self, line_counts};
    use super::thirdparty_task_thread_pool;
    use super::types::{
        field_type, format_type, matrix_market_header, object_type, read_options, symmetry_type,
        write_options,
    };
    use super::write_body;
    use super::Placeholder;
    use std::any::TypeId;
    use std::collections::VecDeque;
    use std::io::{BufRead, Write};

    pub trait triplet_value_type: Clone + Send + 'static {
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

    /// Translation of `generalize_symmetry_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:45`.
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

    /// Translation of `read_matrix_market_body_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:84`.
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
        let threads = app_options.parallel_ok && app_options.num_threads != 1;
        if threads
            && TypeId::of::<V>() == TypeId::of::<()>()
            && header.format == format_type::coordinate
            && header.object == object_type::matrix
            && header.field == field_type::pattern
            && (header.symmetry == symmetry_type::general || app_generalize)
        {
            let requested_threads = if app_options.num_threads < 1 {
                std::thread::available_parallelism()
                    .map(|count| count.get())
                    .unwrap_or(1)
            } else {
                app_options.num_threads as usize
            };
            let inflight_count = requested_threads.saturating_add(1).max(1);
            let mut pool =
                thirdparty_task_thread_pool::task_thread_pool_line_137(requested_threads);
            let mut futures: VecDeque<
                std::sync::mpsc::Receiver<Result<(), fast_matrix_market::invalid_mm>>,
            > = VecDeque::new();
            let mut lc = line_counts {
                file_line: header.header_line_count,
                element_num: 0,
            };
            let mut direct_rows = vec![0i64; base_nnz];
            let mut direct_cols = vec![0i64; base_nnz];
            let rows_addr = direct_rows.as_mut_ptr() as usize;
            let cols_addr = direct_cols.as_mut_ptr() as usize;

            loop {
                while futures.len() >= inflight_count {
                    match futures.pop_front().unwrap().recv().unwrap() {
                        Ok(()) => {}
                        Err(err) => {
                            thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                            return Err(err);
                        }
                    }
                }

                let chunk = chunking::get_next_chunk_line_51(instream, &app_options)
                    .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
                if chunk.is_empty() {
                    break;
                }
                let counted = super::read_body_threads::count_chunk_lines_line_24(chunk);
                if lc.element_num > header.nnz {
                    thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                    return Err(fast_matrix_market::invalid_mm_line_55(
                        "File too long".to_string(),
                        lc.file_line + 1,
                    ));
                }
                let start_lc = lc.clone();
                let counts = counted.counts;
                lc.file_line += counts.file_line;
                lc.element_num += counts.element_num;
                let header_for_task = header.clone();
                futures.push_back(thirdparty_task_thread_pool::std_future_r_submit_line_248(
                    &mut pool,
                    move || {
                        let rows_ptr = rows_addr as *mut i64;
                        let cols_ptr = cols_addr as *mut i64;
                        let mut line = start_lc;
                        let mut offset = line.element_num.max(0) as usize;
                        let parse_integer_field = |field: &str| -> Result<i64, ()> {
                            let bytes = field.as_bytes();
                            let mut pos = 0usize;
                            let negative = pos < bytes.len() && bytes[pos] == b'-';
                            if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
                                pos += 1;
                            }
                            if pos == bytes.len() {
                                return Err(());
                            }
                            let mut value = 0i64;
                            while pos < bytes.len() {
                                let byte = bytes[pos];
                                if !byte.is_ascii_digit() {
                                    return Err(());
                                }
                                value = value
                                    .checked_mul(10)
                                    .and_then(|value| value.checked_add((byte - b'0') as i64))
                                    .ok_or(())?;
                                pos += 1;
                            }
                            if negative {
                                value.checked_neg().ok_or(())
                            } else {
                                Ok(value)
                            }
                        };

                        let chunk_bytes = counted.chunk.as_bytes();
                        let mut raw_start = 0usize;
                        while raw_start < chunk_bytes.len() {
                            let mut raw_end = raw_start;
                            while raw_end < chunk_bytes.len() && chunk_bytes[raw_end] != b'\n' {
                                raw_end += 1;
                            }
                            if raw_end < chunk_bytes.len() {
                                raw_end += 1;
                            }
                            let raw_line = &counted.chunk[raw_start..raw_end];
                            raw_start = raw_end;
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
                                && (bytes[pos] == b' '
                                    || bytes[pos] == b'\t'
                                    || bytes[pos] == b'\r')
                            {
                                pos += 1;
                            }
                            if pos == end {
                                line.file_line +=
                                    bytes.iter().filter(|&&c| c == b'\n').count() as i64;
                                continue;
                            }
                            if line.element_num >= header_for_task.nnz {
                                return Err(fast_matrix_market::invalid_mm_line_55(
                                    "Too many lines in file (file too long)".to_string(),
                                    line.file_line + 1,
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
                            let row_one = parse_integer_field(row_field).map_err(|_| {
                                fast_matrix_market::invalid_mm_line_55(
                                    "Invalid integer value.".to_string(),
                                    line.file_line + 1,
                                )
                            })?;
                            while pos < end
                                && (bytes[pos] == b' '
                                    || bytes[pos] == b'\t'
                                    || bytes[pos] == b'\r')
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
                            let col_one = parse_integer_field(col_field).map_err(|_| {
                                fast_matrix_market::invalid_mm_line_55(
                                    "Invalid integer value.".to_string(),
                                    line.file_line + 1,
                                )
                            })?;
                            if row_one <= 0 || row_one > header_for_task.nrows {
                                return Err(fast_matrix_market::invalid_mm_line_55(
                                    "Row index out of bounds".to_string(),
                                    line.file_line + 1,
                                ));
                            }
                            if col_one <= 0 || col_one > header_for_task.ncols {
                                return Err(fast_matrix_market::invalid_mm_line_55(
                                    "Column index out of bounds".to_string(),
                                    line.file_line + 1,
                                ));
                            }
                            unsafe {
                                *rows_ptr.add(offset) = row_one - 1;
                                *cols_ptr.add(offset) = col_one - 1;
                            }
                            offset += 1;
                            line.file_line += 1;
                            line.element_num += 1;
                        }
                        Ok(())
                    },
                ));
            }

            while let Some(rx) = futures.pop_front() {
                match rx.recv().unwrap() {
                    Ok(()) => {}
                    Err(err) => {
                        thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                        return Err(err);
                    }
                }
            }
            thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
            if lc.element_num < header.nnz {
                return Err(fast_matrix_market::invalid_mm_line_54(format!(
                    "Truncated file. Expected another {} lines.",
                    header.nnz - lc.element_num
                )));
            }
            let mut values: Vec<V> = (0..base_nnz).map(|_| V::pattern_value()).collect();
            if app_generalize && header.symmetry != symmetry_type::general {
                let orig_size = direct_rows.len();
                for i in 0..orig_size {
                    if direct_rows[i] == direct_cols[i] {
                        continue;
                    }
                    direct_rows.push(direct_cols[i]);
                    direct_cols.push(direct_rows[i]);
                    let value = values[i].symmetric_value(header.symmetry);
                    values.push(value);
                }
            }
            return Ok((direct_rows, direct_cols, values));
        }
        if threads
            && TypeId::of::<V>() == TypeId::of::<f64>()
            && header.format == format_type::coordinate
            && (header.field == field_type::real || header.field == field_type::integer)
            && (header.symmetry == symmetry_type::general || app_generalize)
        {
            let requested_threads = if app_options.num_threads < 1 {
                std::thread::available_parallelism()
                    .map(|count| count.get())
                    .unwrap_or(1)
            } else {
                app_options.num_threads as usize
            };
            let inflight_count = requested_threads.saturating_add(1).max(1);
            let mut pool =
                thirdparty_task_thread_pool::task_thread_pool_line_137(requested_threads);
            let mut futures: VecDeque<
                std::sync::mpsc::Receiver<Result<(), fast_matrix_market::invalid_mm>>,
            > = VecDeque::new();
            let mut lc = line_counts {
                file_line: header.header_line_count,
                element_num: 0,
            };
            let mut direct_rows = vec![0i64; base_nnz];
            let mut direct_cols = vec![0i64; base_nnz];
            let mut direct_values = vec![0f64; base_nnz];
            let rows_addr = direct_rows.as_mut_ptr() as usize;
            let cols_addr = direct_cols.as_mut_ptr() as usize;
            let values_addr = direct_values.as_mut_ptr() as usize;

            loop {
                while futures.len() >= inflight_count {
                    match futures.pop_front().unwrap().recv().unwrap() {
                        Ok(()) => {}
                        Err(err) => {
                            thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                            return Err(err);
                        }
                    }
                }

                let chunk = chunking::get_next_chunk_line_51(instream, &app_options)
                    .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
                if chunk.is_empty() {
                    break;
                }
                let counted = super::read_body_threads::count_chunk_lines_line_24(chunk);
                if lc.element_num > header.nnz {
                    thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                    return Err(fast_matrix_market::invalid_mm_line_55(
                        "File too long".to_string(),
                        lc.file_line + 1,
                    ));
                }
                let start_lc = lc.clone();
                let counts = counted.counts;
                lc.file_line += counts.file_line;
                lc.element_num += counts.element_num;
                let header_for_task = header.clone();
                let options_for_task = app_options.clone();
                futures.push_back(thirdparty_task_thread_pool::std_future_r_submit_line_248(
                    &mut pool,
                    move || {
                        let rows_ptr = rows_addr as *mut i64;
                        let cols_ptr = cols_addr as *mut i64;
                        let values_ptr = values_addr as *mut f64;
                        let mut line = start_lc;
                        let mut offset = line.element_num.max(0) as usize;
                        let parse_integer_field = |field: &str| -> Result<i64, ()> {
                            let bytes = field.as_bytes();
                            let mut pos = 0usize;
                            let negative = pos < bytes.len() && bytes[pos] == b'-';
                            if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
                                pos += 1;
                            }
                            if pos == bytes.len() {
                                return Err(());
                            }
                            let mut value = 0i64;
                            while pos < bytes.len() {
                                let byte = bytes[pos];
                                if !byte.is_ascii_digit() {
                                    return Err(());
                                }
                                value = value
                                    .checked_mul(10)
                                    .and_then(|value| value.checked_add((byte - b'0') as i64))
                                    .ok_or(())?;
                                pos += 1;
                            }
                            if negative {
                                value.checked_neg().ok_or(())
                            } else {
                                Ok(value)
                            }
                        };

                        let chunk_bytes = counted.chunk.as_bytes();
                        let mut raw_start = 0usize;
                        while raw_start < chunk_bytes.len() {
                            let mut raw_end = raw_start;
                            while raw_end < chunk_bytes.len() && chunk_bytes[raw_end] != b'\n' {
                                raw_end += 1;
                            }
                            if raw_end < chunk_bytes.len() {
                                raw_end += 1;
                            }
                            let raw_line = &counted.chunk[raw_start..raw_end];
                            raw_start = raw_end;
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
                                && (bytes[pos] == b' '
                                    || bytes[pos] == b'\t'
                                    || bytes[pos] == b'\r')
                            {
                                pos += 1;
                            }
                            if pos == end {
                                line.file_line +=
                                    bytes.iter().filter(|&&c| c == b'\n').count() as i64;
                                continue;
                            }
                            if line.element_num >= header_for_task.nnz {
                                return Err(fast_matrix_market::invalid_mm_line_55(
                                    "Too many lines in file (file too long)".to_string(),
                                    line.file_line + 1,
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
                            let row_one = parse_integer_field(row_field).map_err(|_| {
                                fast_matrix_market::invalid_mm_line_55(
                                    "Invalid integer value.".to_string(),
                                    line.file_line + 1,
                                )
                            })?;
                            let (row_zero, col_zero) =
                                if header_for_task.object == object_type::matrix {
                                    while pos < end
                                        && (bytes[pos] == b' '
                                            || bytes[pos] == b'\t'
                                            || bytes[pos] == b'\r')
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
                                    let col_one = parse_integer_field(col_field).map_err(|_| {
                                        fast_matrix_market::invalid_mm_line_55(
                                            "Invalid integer value.".to_string(),
                                            line.file_line + 1,
                                        )
                                    })?;
                                    if row_one <= 0 || row_one > header_for_task.nrows {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Row index out of bounds".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    if col_one <= 0 || col_one > header_for_task.ncols {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Column index out of bounds".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    (row_one - 1, col_one - 1)
                                } else {
                                    if row_one <= 0 || row_one > header_for_task.vector_length {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Vector index out of bounds".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    (row_one - 1, 0)
                                };
                            while pos < end
                                && (bytes[pos] == b' '
                                    || bytes[pos] == b'\t'
                                    || bytes[pos] == b'\r')
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
                                    line.file_line + 1,
                                ));
                            }
                            let value_field = &raw_line[value_start..pos];
                            let value = read_body::read_real_or_complex_line_193::<f64>(
                                &[value_field],
                                &header_for_task,
                                &options_for_task,
                            )
                            .map_err(|err| {
                                fast_matrix_market::invalid_mm_line_55(err.msg, line.file_line + 1)
                            })?;
                            unsafe {
                                *rows_ptr.add(offset) = row_zero;
                                *cols_ptr.add(offset) = col_zero;
                                *values_ptr.add(offset) = value;
                            }
                            offset += 1;
                            line.file_line += 1;
                            line.element_num += 1;
                        }
                        Ok(())
                    },
                ));
            }

            while let Some(rx) = futures.pop_front() {
                match rx.recv().unwrap() {
                    Ok(()) => {}
                    Err(err) => {
                        thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                        return Err(err);
                    }
                }
            }
            thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
            if lc.element_num < header.nnz {
                return Err(fast_matrix_market::invalid_mm_line_54(format!(
                    "Truncated file. Expected another {} lines.",
                    header.nnz - lc.element_num
                )));
            }
            let mut values = unsafe {
                let mut values = std::mem::ManuallyDrop::new(direct_values);
                Vec::from_raw_parts(
                    values.as_mut_ptr() as *mut V,
                    values.len(),
                    values.capacity(),
                )
            };
            if app_generalize && header.symmetry != symmetry_type::general {
                let orig_size = direct_rows.len();
                for i in 0..orig_size {
                    if direct_rows[i] == direct_cols[i] {
                        continue;
                    }
                    direct_rows.push(direct_cols[i]);
                    direct_cols.push(direct_rows[i]);
                    let value = values[i].symmetric_value(header.symmetry);
                    values.push(value);
                }
            }
            return Ok((direct_rows, direct_cols, values));
        }
        if threads
            && TypeId::of::<V>() == TypeId::of::<(f64, f64)>()
            && header.format == format_type::coordinate
            && header.field == field_type::complex
            && (header.symmetry == symmetry_type::general || app_generalize)
        {
            let requested_threads = if app_options.num_threads < 1 {
                std::thread::available_parallelism()
                    .map(|count| count.get())
                    .unwrap_or(1)
            } else {
                app_options.num_threads as usize
            };
            let inflight_count = requested_threads.saturating_add(1).max(1);
            let mut pool =
                thirdparty_task_thread_pool::task_thread_pool_line_137(requested_threads);
            let mut futures: VecDeque<
                std::sync::mpsc::Receiver<Result<(), fast_matrix_market::invalid_mm>>,
            > = VecDeque::new();
            let mut lc = line_counts {
                file_line: header.header_line_count,
                element_num: 0,
            };
            let mut direct_rows = vec![0i64; base_nnz];
            let mut direct_cols = vec![0i64; base_nnz];
            let mut direct_values = vec![(0f64, 0f64); base_nnz];
            let rows_addr = direct_rows.as_mut_ptr() as usize;
            let cols_addr = direct_cols.as_mut_ptr() as usize;
            let values_addr = direct_values.as_mut_ptr() as usize;

            loop {
                while futures.len() >= inflight_count {
                    match futures.pop_front().unwrap().recv().unwrap() {
                        Ok(()) => {}
                        Err(err) => {
                            thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                            return Err(err);
                        }
                    }
                }

                let chunk = chunking::get_next_chunk_line_51(instream, &app_options)
                    .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
                if chunk.is_empty() {
                    break;
                }
                let counted = super::read_body_threads::count_chunk_lines_line_24(chunk);
                if lc.element_num > header.nnz {
                    thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                    return Err(fast_matrix_market::invalid_mm_line_55(
                        "File too long".to_string(),
                        lc.file_line + 1,
                    ));
                }
                let start_lc = lc.clone();
                let counts = counted.counts;
                lc.file_line += counts.file_line;
                lc.element_num += counts.element_num;
                let header_for_task = header.clone();
                let options_for_task = app_options.clone();
                futures.push_back(thirdparty_task_thread_pool::std_future_r_submit_line_248(
                    &mut pool,
                    move || {
                        let rows_ptr = rows_addr as *mut i64;
                        let cols_ptr = cols_addr as *mut i64;
                        let values_ptr = values_addr as *mut (f64, f64);
                        let mut line = start_lc;
                        let mut offset = line.element_num.max(0) as usize;
                        let parse_integer_field = |field: &str| -> Result<i64, ()> {
                            let bytes = field.as_bytes();
                            let mut pos = 0usize;
                            let negative = pos < bytes.len() && bytes[pos] == b'-';
                            if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
                                pos += 1;
                            }
                            if pos == bytes.len() {
                                return Err(());
                            }
                            let mut value = 0i64;
                            while pos < bytes.len() {
                                let byte = bytes[pos];
                                if !byte.is_ascii_digit() {
                                    return Err(());
                                }
                                value = value
                                    .checked_mul(10)
                                    .and_then(|value| value.checked_add((byte - b'0') as i64))
                                    .ok_or(())?;
                                pos += 1;
                            }
                            if negative {
                                value.checked_neg().ok_or(())
                            } else {
                                Ok(value)
                            }
                        };

                        let chunk_bytes = counted.chunk.as_bytes();
                        let mut raw_start = 0usize;
                        while raw_start < chunk_bytes.len() {
                            let mut raw_end = raw_start;
                            while raw_end < chunk_bytes.len() && chunk_bytes[raw_end] != b'\n' {
                                raw_end += 1;
                            }
                            if raw_end < chunk_bytes.len() {
                                raw_end += 1;
                            }
                            let raw_line = &counted.chunk[raw_start..raw_end];
                            raw_start = raw_end;
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
                                && (bytes[pos] == b' '
                                    || bytes[pos] == b'\t'
                                    || bytes[pos] == b'\r')
                            {
                                pos += 1;
                            }
                            if pos == end {
                                line.file_line +=
                                    bytes.iter().filter(|&&c| c == b'\n').count() as i64;
                                continue;
                            }
                            if line.element_num >= header_for_task.nnz {
                                return Err(fast_matrix_market::invalid_mm_line_55(
                                    "Too many lines in file (file too long)".to_string(),
                                    line.file_line + 1,
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
                            let row_one = parse_integer_field(row_field).map_err(|_| {
                                fast_matrix_market::invalid_mm_line_55(
                                    "Invalid integer value.".to_string(),
                                    line.file_line + 1,
                                )
                            })?;
                            let (row_zero, col_zero) =
                                if header_for_task.object == object_type::matrix {
                                    while pos < end
                                        && (bytes[pos] == b' '
                                            || bytes[pos] == b'\t'
                                            || bytes[pos] == b'\r')
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
                                    let col_one = parse_integer_field(col_field).map_err(|_| {
                                        fast_matrix_market::invalid_mm_line_55(
                                            "Invalid integer value.".to_string(),
                                            line.file_line + 1,
                                        )
                                    })?;
                                    if row_one <= 0 || row_one > header_for_task.nrows {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Row index out of bounds".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    if col_one <= 0 || col_one > header_for_task.ncols {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Column index out of bounds".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    (row_one - 1, col_one - 1)
                                } else {
                                    if row_one <= 0 || row_one > header_for_task.vector_length {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Vector index out of bounds".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    (row_one - 1, 0)
                                };
                            while pos < end
                                && (bytes[pos] == b' '
                                    || bytes[pos] == b'\t'
                                    || bytes[pos] == b'\r')
                            {
                                pos += 1;
                            }
                            let real_start = pos;
                            while pos < end
                                && bytes[pos] != b' '
                                && bytes[pos] != b'\t'
                                && bytes[pos] != b'\r'
                                && bytes[pos] != b'\n'
                            {
                                pos += 1;
                            }
                            if real_start == pos {
                                return Err(fast_matrix_market::invalid_mm_line_55(
                                    "Invalid floating-point value.".to_string(),
                                    line.file_line + 1,
                                ));
                            }
                            let real_end = pos;
                            while pos < end
                                && (bytes[pos] == b' '
                                    || bytes[pos] == b'\t'
                                    || bytes[pos] == b'\r')
                            {
                                pos += 1;
                            }
                            let imaginary_start = pos;
                            while pos < end
                                && bytes[pos] != b' '
                                && bytes[pos] != b'\t'
                                && bytes[pos] != b'\r'
                                && bytes[pos] != b'\n'
                            {
                                pos += 1;
                            }
                            if imaginary_start == pos {
                                return Err(fast_matrix_market::invalid_mm_line_55(
                                    "Invalid floating-point value.".to_string(),
                                    line.file_line + 1,
                                ));
                            }
                            let real_field = &raw_line[real_start..real_end];
                            let imaginary_field = &raw_line[imaginary_start..pos];
                            let value = read_body::read_real_or_complex_line_193::<(f64, f64)>(
                                &[real_field, imaginary_field],
                                &header_for_task,
                                &options_for_task,
                            )
                            .map_err(|err| {
                                fast_matrix_market::invalid_mm_line_55(err.msg, line.file_line + 1)
                            })?;
                            unsafe {
                                *rows_ptr.add(offset) = row_zero;
                                *cols_ptr.add(offset) = col_zero;
                                *values_ptr.add(offset) = value;
                            }
                            offset += 1;
                            line.file_line += 1;
                            line.element_num += 1;
                        }
                        Ok(())
                    },
                ));
            }

            while let Some(rx) = futures.pop_front() {
                match rx.recv().unwrap() {
                    Ok(()) => {}
                    Err(err) => {
                        thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                        return Err(err);
                    }
                }
            }
            thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
            if lc.element_num < header.nnz {
                return Err(fast_matrix_market::invalid_mm_line_54(format!(
                    "Truncated file. Expected another {} lines.",
                    header.nnz - lc.element_num
                )));
            }
            let mut values = unsafe {
                let mut values = std::mem::ManuallyDrop::new(direct_values);
                Vec::from_raw_parts(
                    values.as_mut_ptr() as *mut V,
                    values.len(),
                    values.capacity(),
                )
            };
            if app_generalize && header.symmetry != symmetry_type::general {
                let orig_size = direct_rows.len();
                for i in 0..orig_size {
                    if direct_rows[i] == direct_cols[i] {
                        continue;
                    }
                    direct_rows.push(direct_cols[i]);
                    direct_cols.push(direct_rows[i]);
                    let value = values[i].symmetric_value(header.symmetry);
                    values.push(value);
                }
            }
            return Ok((direct_rows, direct_cols, values));
        }
        if header.format == format_type::coordinate
            && (header.symmetry == symmetry_type::general || !app_options.generalize_symmetry)
            && !threads
        {
            if TypeId::of::<V>() == TypeId::of::<f64>()
                && (header.field == field_type::real || header.field == field_type::integer)
            {
                let mut lc = line_counts {
                    file_line: header.header_line_count,
                    element_num: 0,
                };
                let mut direct_rows = vec![0i64; base_nnz];
                let mut direct_cols = vec![0i64; base_nnz];
                let mut direct_values = vec![0f64; base_nnz];
                let mut offset = 0usize;
                let parse_integer_field = |field: &str| -> Result<i64, ()> {
                    let bytes = field.as_bytes();
                    let mut pos = 0usize;
                    let negative = pos < bytes.len() && bytes[pos] == b'-';
                    if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
                        pos += 1;
                    }
                    if pos == bytes.len() {
                        return Err(());
                    }
                    let mut value = 0i64;
                    while pos < bytes.len() {
                        let byte = bytes[pos];
                        if !byte.is_ascii_digit() {
                            return Err(());
                        }
                        value = value
                            .checked_mul(10)
                            .and_then(|value| value.checked_add((byte - b'0') as i64))
                            .ok_or(())?;
                        pos += 1;
                    }
                    if negative {
                        value.checked_neg().ok_or(())
                    } else {
                        Ok(value)
                    }
                };

                loop {
                    let chunk =
                        chunking::get_next_chunk_line_51(instream, &app_options).map_err(|_| {
                            fast_matrix_market::invalid_mm_line_54("I/O error".to_string())
                        })?;
                    if chunk.is_empty() {
                        break;
                    }
                    let chunk_bytes = chunk.as_bytes();
                    let mut raw_start = 0usize;
                    while raw_start < chunk_bytes.len() {
                        let mut raw_end = raw_start;
                        while raw_end < chunk_bytes.len() && chunk_bytes[raw_end] != b'\n' {
                            raw_end += 1;
                        }
                        if raw_end < chunk_bytes.len() {
                            raw_end += 1;
                        }
                        let raw_line = &chunk[raw_start..raw_end];
                        raw_start = raw_end;
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
                        let row_one = parse_integer_field(row_field).map_err(|_| {
                            fast_matrix_market::invalid_mm_line_55(
                                "Invalid integer value.".to_string(),
                                lc.file_line + 1,
                            )
                        })?;
                        let (row_zero, col_zero) = if header.object == object_type::matrix {
                            while pos < end
                                && (bytes[pos] == b' '
                                    || bytes[pos] == b'\t'
                                    || bytes[pos] == b'\r')
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
                            let col_one = parse_integer_field(col_field).map_err(|_| {
                                fast_matrix_market::invalid_mm_line_55(
                                    "Invalid integer value.".to_string(),
                                    lc.file_line + 1,
                                )
                            })?;
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
                        let value_field = &raw_line[value_start..pos];
                        let value = read_body::read_real_or_complex_line_193::<f64>(
                            &[value_field],
                            header,
                            &app_options,
                        )
                        .map_err(|err| {
                            fast_matrix_market::invalid_mm_line_55(err.msg, lc.file_line + 1)
                        })?;
                        direct_rows[offset] = row_zero;
                        direct_cols[offset] = col_zero;
                        direct_values[offset] = value;
                        offset += 1;
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
                let mut values = unsafe {
                    let mut values = std::mem::ManuallyDrop::new(direct_values);
                    Vec::from_raw_parts(
                        values.as_mut_ptr() as *mut V,
                        values.len(),
                        values.capacity(),
                    )
                };
                if app_generalize && header.symmetry != symmetry_type::general {
                    let orig_size = direct_rows.len();
                    for i in 0..orig_size {
                        if direct_rows[i] == direct_cols[i] {
                            continue;
                        }
                        direct_rows.push(direct_cols[i]);
                        direct_cols.push(direct_rows[i]);
                        let value = values[i].symmetric_value(header.symmetry);
                        values.push(value);
                    }
                }
                return Ok((direct_rows, direct_cols, values));
            }
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
        if threads {
            if header.format == format_type::coordinate
                && (header.symmetry == symmetry_type::general || !app_options.generalize_symmetry)
            {
                let requested_threads = if app_options.num_threads < 1 {
                    std::thread::available_parallelism()
                        .map(|count| count.get())
                        .unwrap_or(1)
                } else {
                    app_options.num_threads as usize
                };
                let inflight_count = requested_threads.saturating_add(1).max(1);
                let mut pool =
                    thirdparty_task_thread_pool::task_thread_pool_line_137(requested_threads);
                let mut futures: VecDeque<
                    std::sync::mpsc::Receiver<
                        Result<
                            (line_counts, Vec<i64>, Vec<i64>, Vec<V>),
                            fast_matrix_market::invalid_mm,
                        >,
                    >,
                > = VecDeque::new();
                let mut lc = line_counts {
                    file_line: header.header_line_count,
                    element_num: 0,
                };

                loop {
                    while futures.len() >= inflight_count {
                        match futures.pop_front().unwrap().recv().unwrap() {
                            Ok((_next_lc, mut chunk_rows, mut chunk_cols, mut chunk_values)) => {
                                rows.append(&mut chunk_rows);
                                cols.append(&mut chunk_cols);
                                values.append(&mut chunk_values);
                            }
                            Err(err) => {
                                thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                                return Err(err);
                            }
                        }
                    }

                    let chunk =
                        chunking::get_next_chunk_line_51(instream, &app_options).map_err(|_| {
                            fast_matrix_market::invalid_mm_line_54("I/O error".to_string())
                        })?;
                    if chunk.is_empty() {
                        break;
                    }
                    let counted = super::read_body_threads::count_chunk_lines_line_24(chunk);
                    if lc.element_num > header.nnz {
                        thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                        return Err(fast_matrix_market::invalid_mm_line_55(
                            "File too long".to_string(),
                            lc.file_line + 1,
                        ));
                    }
                    let start_lc = lc.clone();
                    let counts = counted.counts;
                    lc.file_line += counts.file_line;
                    lc.element_num += counts.element_num;
                    let header_for_task = header.clone();
                    let options_for_task = app_options.clone();
                    futures.push_back(thirdparty_task_thread_pool::std_future_r_submit_line_248(
                        &mut pool,
                        move || {
                            let mut line = start_lc;
                            let parse_integer_field = |field: &str| -> Result<i64, ()> {
                                let bytes = field.as_bytes();
                                let mut pos = 0usize;
                                let negative = pos < bytes.len() && bytes[pos] == b'-';
                                if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
                                    pos += 1;
                                }
                                if pos == bytes.len() {
                                    return Err(());
                                }
                                let mut value = 0i64;
                                while pos < bytes.len() {
                                    let byte = bytes[pos];
                                    if !byte.is_ascii_digit() {
                                        return Err(());
                                    }
                                    value = value
                                        .checked_mul(10)
                                        .and_then(|value| value.checked_add((byte - b'0') as i64))
                                        .ok_or(())?;
                                    pos += 1;
                                }
                                if negative {
                                    value.checked_neg().ok_or(())
                                } else {
                                    Ok(value)
                                }
                            };
                            let mut chunk_rows =
                                Vec::with_capacity(counts.element_num.max(0) as usize);
                            let mut chunk_cols =
                                Vec::with_capacity(counts.element_num.max(0) as usize);
                            let mut chunk_values =
                                Vec::with_capacity(counts.element_num.max(0) as usize);
                            let chunk_bytes = counted.chunk.as_bytes();
                            let mut raw_start = 0usize;
                            while raw_start < chunk_bytes.len() {
                                let mut raw_end = raw_start;
                                while raw_end < chunk_bytes.len() && chunk_bytes[raw_end] != b'\n' {
                                    raw_end += 1;
                                }
                                if raw_end < chunk_bytes.len() {
                                    raw_end += 1;
                                }
                                let raw_line = &counted.chunk[raw_start..raw_end];
                                raw_start = raw_end;
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
                                    && (bytes[pos] == b' '
                                        || bytes[pos] == b'\t'
                                        || bytes[pos] == b'\r')
                                {
                                    pos += 1;
                                }
                                if pos == end {
                                    line.file_line +=
                                        bytes.iter().filter(|&&c| c == b'\n').count() as i64;
                                    continue;
                                }
                                if line.element_num >= header_for_task.nnz {
                                    return Err(fast_matrix_market::invalid_mm_line_55(
                                        "Too many lines in file (file too long)".to_string(),
                                        line.file_line + 1,
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
                                let row_one = parse_integer_field(row_field).map_err(|_| {
                                    fast_matrix_market::invalid_mm_line_55(
                                        "Invalid integer value.".to_string(),
                                        line.file_line + 1,
                                    )
                                })?;
                                if row_field.is_empty() {
                                    return Err(fast_matrix_market::invalid_mm_line_55(
                                        "Invalid integer value.".to_string(),
                                        line.file_line + 1,
                                    ));
                                }
                                let (row_zero, col_zero) = if header_for_task.object
                                    == object_type::matrix
                                {
                                    while pos < end
                                        && (bytes[pos] == b' '
                                            || bytes[pos] == b'\t'
                                            || bytes[pos] == b'\r')
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
                                    let col_one = parse_integer_field(col_field).map_err(|_| {
                                        fast_matrix_market::invalid_mm_line_55(
                                            "Invalid integer value.".to_string(),
                                            line.file_line + 1,
                                        )
                                    })?;
                                    if col_field.is_empty() {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Invalid integer value.".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    if row_one <= 0 || row_one > header_for_task.nrows {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Row index out of bounds".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    if col_one <= 0 || col_one > header_for_task.ncols {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Column index out of bounds".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    (row_one - 1, col_one - 1)
                                } else {
                                    if row_one <= 0 || row_one > header_for_task.vector_length {
                                        return Err(fast_matrix_market::invalid_mm_line_55(
                                            "Vector index out of bounds".to_string(),
                                            line.file_line + 1,
                                        ));
                                    }
                                    (row_one - 1, 0)
                                };
                                chunk_rows.push(row_zero);
                                chunk_cols.push(col_zero);
                                if header_for_task.field != field_type::pattern {
                                    while pos < end
                                        && (bytes[pos] == b' '
                                            || bytes[pos] == b'\t'
                                            || bytes[pos] == b'\r')
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
                                            line.file_line + 1,
                                        ));
                                    }
                                    if header_for_task.field == field_type::complex {
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
                                                line.file_line + 1,
                                            ));
                                        }
                                        let imaginary_field = &raw_line[imag_start..pos];
                                        let value = read_body::read_real_or_complex_line_193::<V>(
                                            &[real_field, imaginary_field],
                                            &header_for_task,
                                            &options_for_task,
                                        )
                                        .map_err(|err| {
                                            fast_matrix_market::invalid_mm_line_55(
                                                err.msg,
                                                line.file_line + 1,
                                            )
                                        })?;
                                        chunk_values.push(value);
                                    } else {
                                        let value_field = &raw_line[value_start..pos];
                                        let value = read_body::read_real_or_complex_line_193::<V>(
                                            &[value_field],
                                            &header_for_task,
                                            &options_for_task,
                                        )
                                        .map_err(|err| {
                                            fast_matrix_market::invalid_mm_line_55(
                                                err.msg,
                                                line.file_line + 1,
                                            )
                                        })?;
                                        chunk_values.push(value);
                                    };
                                } else {
                                    chunk_values.push(V::pattern_value());
                                }
                                line.file_line += 1;
                                line.element_num += 1;
                            }
                            Ok((line, chunk_rows, chunk_cols, chunk_values))
                        },
                    ));
                }

                while let Some(rx) = futures.pop_front() {
                    match rx.recv().unwrap() {
                        Ok((_next_lc, mut chunk_rows, mut chunk_cols, mut chunk_values)) => {
                            rows.append(&mut chunk_rows);
                            cols.append(&mut chunk_cols);
                            values.append(&mut chunk_values);
                        }
                        Err(err) => {
                            thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                            return Err(err);
                        }
                    }
                }
                thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
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
            let (lc, entries) = super::read_body_threads::read_body_threads_line_33::<V>(
                instream,
                header,
                &app_options,
            )?;
            rows.reserve(entries.len());
            cols.reserve(entries.len());
            values.reserve(entries.len());
            for (entry_row, entry_col, value) in entries {
                rows.push(entry_row);
                cols.push(entry_col);
                values.push(value);
            }
            if lc.element_num < header.nnz
                && !(header.symmetry != symmetry_type::general
                    && header.format == format_type::array)
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

    /// Translation of `read_matrix_market_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:112`.
    pub fn read_matrix_market_triplet_line_112<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &mut matrix_market_header,
        options: &read_options,
    ) -> Result<(Vec<i64>, Vec<i64>, Vec<V>), super::fast_matrix_market::invalid_mm> {
        header::read_header_line_166(instream, header)?;
        read_matrix_market_body_triplet_line_84::<V>(instream, header, options)
    }

    /// Translation of `read_matrix_market_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:126`.
    pub fn read_matrix_market_triplet_line_126<V: triplet_value_type>(
        instream: &mut impl BufRead,
        options: &read_options,
    ) -> Result<(i64, i64, Vec<i64>, Vec<i64>, Vec<V>), super::fast_matrix_market::invalid_mm> {
        let mut header = super::types::matrix_market_header_line_48();
        let (rows, cols, values) =
            read_matrix_market_triplet_line_112::<V>(instream, &mut header, options)?;
        Ok((header.nrows, header.ncols, rows, cols, values))
    }

    /// Translation of `write_matrix_market_triplet` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:140`.
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

    /// Translation of `write_matrix_market_csc` at `fast_matrix_market/include/fast_matrix_market/app/triplet.hpp:173`.
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

    /// Translation of `read_value` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:38`.
    pub fn read_value_line_38(s: &str, mut pos: usize) -> (usize, String) {
        let field_start = pos;
        let bytes = s.as_bytes();
        while pos < bytes.len() && bytes[pos] != b'\n' {
            pos += 1;
        }
        (pos, s[field_start..pos].to_string())
    }

    /// Translation of `negate` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:57`.
    pub fn negate_line_57(o: &str) -> String {
        format!("-{o}")
    }

    /// Translation of `pattern_default_value` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:64`.
    pub fn pattern_default_value_line_64() -> String {
        String::new()
    }

    /// Translation of `get_field_type` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:76`.
    pub fn get_field_type_line_76() -> field_type {
        field_type::real
    }

    /// Translation of `value_to_string` at `fast_matrix_market/include/fast_matrix_market/app/user_type_string.hpp:83`.
    pub fn value_to_string_line_83(value: &str, _precision: i32) -> String {
        value.to_string()
    }
}

pub mod chunking {
    use super::types::read_options;
    use super::Placeholder;
    use std::io::{BufRead, Read};

    /// Translation of `get_next_chunk` at `fast_matrix_market/include/fast_matrix_market/chunking.hpp:11`.
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

    /// Translation of `get_next_chunk` at `fast_matrix_market/include/fast_matrix_market/chunking.hpp:51`.
    pub fn get_next_chunk_line_51<R: BufRead>(
        instream: &mut R,
        options: &read_options,
    ) -> std::io::Result<String> {
        let mut chunk = String::new();
        get_next_chunk_line_11(&mut chunk, instream, options)?;
        Ok(chunk)
    }

    /// Translation of `is_all_spaces` at `fast_matrix_market/include/fast_matrix_market/chunking.hpp:59`.
    pub fn is_all_spaces_line_59(text: &str) -> bool {
        text.bytes().all(|c| c == b' ' || c == b'\t' || c == b'\r')
    }

    /// Translation of `count_lines` at `fast_matrix_market/include/fast_matrix_market/chunking.hpp:66`.
    pub fn count_lines_line_66(chunk: &str) -> (i64, i64) {
        let mut num_newlines = 0i64;
        let mut num_empty_lines = 0i64;
        let mut line_start = 0usize;
        let bytes = chunk.as_bytes();

        for pos in 0..bytes.len() {
            if bytes[pos] == b'\n' {
                num_newlines += 1;
                if bytes[line_start..pos]
                    .iter()
                    .all(|&c| c == b' ' || c == b'\t' || c == b'\r')
                {
                    num_empty_lines += 1;
                }
                line_start = pos + 1;
            }
        }

        if line_start != bytes.len()
            && bytes[line_start..]
                .iter()
                .all(|&c| c == b' ' || c == b'\t' || c == b'\r')
        {
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

    /// Translation of `fmm_error` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:40`.
    pub fn fmm_error_line_40(msg: String) -> fmm_error {
        fmm_error { msg }
    }

    /// Translation of `what` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:42`.
    pub fn what_line_42(err: &fmm_error) -> &str {
        err.msg.as_str()
    }

    /// Translation of `invalid_mm` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:54`.
    pub fn invalid_mm_line_54(msg: String) -> invalid_mm {
        invalid_mm { msg }
    }

    /// Translation of `invalid_mm` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:55`.
    pub fn invalid_mm_line_55(msg: String, line_num: i64) -> invalid_mm {
        let mut err = invalid_mm { msg };
        prepend_line_number_line_59(&mut err, line_num);
        err
    }

    /// Translation of `prepend_line_number` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:59`.
    pub fn prepend_line_number_line_59(err: &mut invalid_mm, line_num: i64) {
        err.msg = format!("Line {line_num}: {}", err.msg);
    }

    /// Translation of `out_of_range` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:69`.
    pub fn out_of_range_line_69(msg: String) -> out_of_range {
        out_of_range { msg }
    }

    /// Translation of `invalid_argument` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:77`.
    pub fn invalid_argument_line_77(msg: String) -> invalid_argument {
        invalid_argument { msg }
    }

    /// Translation of `complex_incompatible` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:85`.
    pub fn complex_incompatible_line_85(msg: String) -> complex_incompatible {
        complex_incompatible { msg }
    }

    /// Translation of `support_not_selected` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:93`.
    pub fn support_not_selected_line_93(msg: String) -> support_not_selected {
        support_not_selected { msg }
    }

    /// Translation of `no_vector_support` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:101`.
    pub fn no_vector_support_line_101(msg: String) -> no_vector_support {
        no_vector_support { msg }
    }

    /// Translation of `operator-` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:114`.
    pub fn operator_line_114(o: pattern_placeholder_type) -> pattern_placeholder_type {
        o
    }

    /// Translation of `negate` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:119`.
    pub fn negate_line_119(o: bool) -> bool {
        !o
    }

    /// Translation of `negate` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:123`.
    pub fn negate_line_123(o: bool) -> bool {
        !o
    }

    /// Translation of `negate` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:128`.
    pub fn negate_line_128<T>(o: T) -> T
    where
        T: std::ops::Neg<Output = T>,
    {
        -o
    }

    /// Translation of `pattern_default_value` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:133`.
    pub fn pattern_default_value_line_133<T>() -> T
    where
        T: From<u8>,
    {
        1u8.into()
    }

    /// Translation of `get_zero` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:141`.
    pub fn get_zero_line_141<T>() -> T
    where
        T: Default,
    {
        T::default()
    }

    /// Translation of `is_ready` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:150`.
    pub fn is_ready_line_150<R>(f: &std::sync::mpsc::Receiver<R>) -> bool {
        match f.try_recv() {
            Ok(_) => true,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => true,
            Err(std::sync::mpsc::TryRecvError::Empty) => false,
        }
    }

    /// Translation of `test_flag` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:160`.
    pub fn test_flag_line_160(flags: i32, flag: i32) -> bool {
        (flags & flag) == flag
    }

    /// Translation of `starts_with` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:164`.
    pub fn starts_with_line_164(str_: &str, prefix: &str) -> bool {
        str_.starts_with(prefix)
    }

    /// Translation of `ends_with` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:171`.
    pub fn ends_with_line_171(str_: &str, suffix: &str) -> bool {
        str_.ends_with(suffix)
    }

    /// Translation of `trim` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:181`.
    pub fn trim_line_181(s: String) -> String {
        s.trim().to_string()
    }

    /// Translation of `replace_all` at `fast_matrix_market/include/fast_matrix_market/fast_matrix_market.hpp:198`.
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

    /// Translation of `skip_spaces` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:34`.
    pub fn skip_spaces_line_34(s: &str, mut pos: usize) -> usize {
        let bytes = s.as_bytes();
        while pos < bytes.len()
            && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r')
        {
            pos += 1;
        }
        pos
    }

    /// Translation of `skip_spaces_and_newlines` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:38`.
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

    /// Translation of `bump_to_next_line` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:48`.
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

    /// Translation of `read_int_from_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:72`.
    pub fn read_int_from_chars_line_72(
        s: &str,
        pos: usize,
        end: usize,
    ) -> Result<(usize, i64), String> {
        read_int_line_140(&s[..end.min(s.len())], pos)
    }

    /// Translation of `read_int_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:85`.
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

    /// Translation of `read_int_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:101`.
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

    /// Translation of `read_int_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:122`.
    pub fn read_int_fallback_line_122(
        s: &str,
        pos: usize,
        end: usize,
    ) -> Result<(usize, i64), String> {
        read_int_fallback_line_85(s, pos, end)
    }

    /// Translation of `read_int` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:140`.
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

    /// Translation of `read_float_fast_float` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:153`.
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

    /// Translation of `read_float_from_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:175`.
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

    /// Translation of `read_float_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:196`.
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

    /// Translation of `read_float_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:216`.
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

    /// Translation of `read_float` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:234`.
    pub fn read_float_line_234(s: &str, pos: usize) -> Result<(usize, f64), String> {
        read_float_fallback_line_196(s, pos)
    }

    /// Translation of `read_float_from_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:257`.
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

    /// Translation of `read_float_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:280`.
    pub fn read_float_fallback_line_280(s: &str, pos: usize) -> Result<(usize, f64), String> {
        read_float_fallback_line_196(s, pos)
    }

    /// Translation of `read_float` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:297`.
    pub fn read_float_line_297(s: &str, pos: usize) -> Result<(usize, f64), String> {
        read_float_fallback_line_280(s, pos)
    }

    /// Translation of `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:312`.
    pub fn read_value_line_312(pos: usize) -> usize {
        pos
    }

    /// Translation of `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:317`.
    pub fn read_value_line_317(s: &str, pos: usize) -> Result<(usize, i64), String> {
        read_int_line_140(s, pos)
    }

    /// Translation of `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:321`.
    pub fn read_value_line_321(s: &str, pos: usize) -> Result<(usize, bool), String> {
        let (next, parsed) = read_float_line_234(s, pos)?;
        Ok((next, parsed != 0.0))
    }

    /// Translation of `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:329`.
    pub fn read_value_line_329(s: &str, pos: usize) -> Result<(usize, f64), String> {
        read_float_line_234(s, pos)
    }

    /// Translation of `read_value` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:334`.
    pub fn read_value_line_334(s: &str, pos: usize) -> Result<(usize, (f64, f64)), String> {
        let (pos, real) = read_float_line_234(s, pos)?;
        let pos = skip_spaces_line_34(s, pos);
        let (pos, imaginary) = read_float_line_234(s, pos)?;
        Ok((pos, (real, imaginary)))
    }

    /// Translation of `complex_conjugate` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:347`.
    pub fn complex_conjugate_line_347(value: (f64, f64)) -> (f64, f64) {
        (value.0, -value.1)
    }

    /// Translation of `complex_conjugate` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:352`.
    pub fn complex_conjugate_line_352<T>(value: T) -> T {
        value
    }

    /// Translation of `int_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:367`.
    pub fn int_to_string_line_367<T: ToString>(value: T) -> String {
        value.to_string()
    }

    /// Translation of `int_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:382`.
    pub fn int_to_string_line_382<T: ToString>(value: T) -> String {
        value.to_string()
    }

    /// Translation of `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:387`.
    pub fn value_to_string_line_387() -> String {
        String::new()
    }

    /// Translation of `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:391`.
    pub fn value_to_string_line_391(value: bool) -> String {
        if value {
            "1".to_string()
        } else {
            "0".to_string()
        }
    }

    /// Translation of `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:396`.
    pub fn value_to_string_line_396<T: ToString>(value: T) -> String {
        int_to_string_line_367(value)
    }

    /// Translation of `value_to_string_fallback` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:404`.
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

    /// Translation of `value_to_string_dragonbox` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:438`.
    pub fn value_to_string_dragonbox_line_438(value: f32) -> String {
        value.to_string()
    }

    /// Translation of `value_to_string_dragonbox` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:452`.
    pub fn value_to_string_dragonbox_line_452(value: f64) -> String {
        value.to_string()
    }

    /// Translation of `value_to_string_ryu` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:468`.
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

    /// Translation of `value_to_string_ryu` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:501`.
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

    /// Translation of `value_to_string_to_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:537`.
    pub fn value_to_string_to_chars_line_537(value: f64, precision: i32) -> String {
        if precision < 0 {
            value.to_string()
        } else {
            value_to_string_fallback_line_404(value, precision)
        }
    }

    /// Translation of `value_to_string_to_chars` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:558`.
    pub fn value_to_string_to_chars_line_558(value: f64, precision: i32) -> String {
        if precision < 0 {
            value.to_string()
        } else {
            value_to_string_fallback_line_404(value, precision)
        }
    }

    /// Translation of `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:584`.
    pub fn value_to_string_line_584(value: f64, precision: i32) -> String {
        value_to_string_fallback_line_404(value, precision)
    }

    /// Translation of `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:601`.
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

    /// Translation of `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:630`.
    pub fn value_to_string_line_630(value: (f64, f64), precision: i32) -> String {
        format!(
            "{} {}",
            value_to_string_line_601(value.0, precision),
            value_to_string_line_601(value.1, precision)
        )
    }

    /// Translation of `value_to_string` at `fast_matrix_market/include/fast_matrix_market/field_conv.hpp:638`.
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

    /// Translation of `line_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:20`.
    pub fn line_formatter_line_20(
        header: matrix_market_header,
        options: write_options,
    ) -> line_formatter {
        line_formatter { header, options }
    }

    /// Translation of `coord_matrix` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:23`.
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

    /// Translation of `coord_matrix_pattern` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:42`.
    pub fn coord_matrix_pattern_line_42(row: i64, col: i64) -> String {
        format!("{} {}\n", row + 1, col + 1)
    }

    /// Translation of `array_matrix` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:52`.
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

    /// Translation of `vector_line_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:79`.
    pub fn vector_line_formatter_line_79(
        header: matrix_market_header,
        options: write_options,
    ) -> vector_line_formatter {
        vector_line_formatter { header, options }
    }

    /// Translation of `coord_matrix` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:81`.
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

    /// Translation of `coord_matrix_pattern` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:94`.
    pub fn coord_matrix_pattern_line_94(row: i64, _col: i64) -> String {
        format!("{}\n", row + 1)
    }

    /// Translation of `triplet_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:120`.
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

    /// Translation of `has_next` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:134`.
    pub fn has_next_line_134<V>(formatter: &triplet_formatter<V>) -> bool {
        formatter.cursor != formatter.rows.len()
    }

    /// Translation of `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:140`.
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

    /// Translation of `operator()` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:149`.
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

    /// Translation of `next_chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:171`.
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

    /// Translation of `csc_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:202`.
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

    /// Translation of `has_next` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:221`.
    pub fn has_next_line_221<V>(formatter: &csc_formatter<V>) -> bool {
        formatter.ptr_iter + 1 < formatter.ptrs.len()
    }

    /// Translation of `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:227`.
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

    /// Translation of `operator()` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:238`.
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

    /// Translation of `next_chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:282`.
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

    /// Translation of `array_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:314`.
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

    /// Translation of `has_next` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:317`.
    pub fn has_next_line_317<V>(formatter: &array_formatter<V>) -> bool {
        formatter.cur_col != formatter.ncols
    }

    /// Translation of `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:323`.
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

    /// Translation of `operator()` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:326`.
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

    /// Translation of `next_chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:351`.
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

    /// Translation of `dense_2d_call_formatter` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:372`.
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

    /// Translation of `has_next` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:375`.
    pub fn has_next_line_375<V>(formatter: &dense_2d_call_formatter<V>) -> bool {
        formatter.col_iter < formatter.ncols
    }

    /// Translation of `chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:381`.
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

    /// Translation of `operator()` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:384`.
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

    /// Translation of `next_chunk` at `fast_matrix_market/include/fast_matrix_market/formatters.hpp:406`.
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

    /// Translation of `parse_enum` at `fast_matrix_market/include/fast_matrix_market/header.hpp:24`.
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

    /// Translation of `is_line_all_spaces` at `fast_matrix_market/include/fast_matrix_market/header.hpp:45`.
    pub fn is_line_all_spaces_line_45(line: &str) -> bool {
        let line = line.strip_suffix('\n').unwrap_or(line);
        chunking::is_all_spaces_line_59(line)
    }

    /// Translation of `strip_trailing_cr` at `fast_matrix_market/include/fast_matrix_market/header.hpp:60`.
    pub fn strip_trailing_cr_line_60(line: &mut String) {
        if line.ends_with('\r') {
            line.pop();
        }
    }

    /// Translation of `get_storage_nnz` at `fast_matrix_market/include/fast_matrix_market/header.hpp:71`.
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

    /// Translation of `read_comment` at `fast_matrix_market/include/fast_matrix_market/header.hpp:116`.
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

    /// Translation of `parse_header_enum` at `fast_matrix_market/include/fast_matrix_market/header.hpp:145`.
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

    /// Translation of `read_header` at `fast_matrix_market/include/fast_matrix_market/header.hpp:166`.
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

    /// Translation of `write_header` at `fast_matrix_market/include/fast_matrix_market/header.hpp:278`.
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

    /// Translation of `tuple_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:50`.
    pub fn tuple_parse_handler_line_50<V: Clone + Default>(iter: usize) -> tuple_parse_handler<V> {
        tuple_parse_handler {
            flags: 1,
            begin_iter: iter,
            iter,
            tuples: Vec::new(),
        }
    }

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:52`.
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

    /// Translation of `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:57`.
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

    /// Translation of `triplet_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:76`.
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

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:81`.
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

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:91`.
    pub fn handle_line_91<V: Clone + Default>(
        handler: &mut triplet_parse_handler<V>,
        row: i64,
        col: i64,
    ) {
        handle_line_81(handler, row, col, V::default());
    }

    /// Translation of `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:100`.
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

    /// Translation of `triplet_pattern_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:126`.
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

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:130`.
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

    /// Translation of `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:138`.
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

    /// Translation of `triplet_calling_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:160`.
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

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:165`.
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

    /// Translation of `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:173`.
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

    /// Translation of `doublet_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:195`.
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

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:199`.
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

    /// Translation of `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:207`.
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

    /// Translation of `dense_2d_call_adding_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:229`.
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

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:231`.
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

    /// Translation of `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:235`.
    pub fn get_chunk_handler_line_235<V: dense_parse_value>(
        handler: &dense_2d_call_adding_parse_handler<V>,
        _offset_from_begin: i64,
    ) -> dense_2d_call_adding_parse_handler<V> {
        handler.clone()
    }

    /// Translation of `dense_adding_parse_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:253`.
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

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:256`.
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

    /// Translation of `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/parse_handlers.hpp:266`.
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

    /// Translation of `pattern_parse_adapter` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:32`.
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

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:35`.
    pub fn handle_line_35<V: Clone + Default>(
        adapter: &mut pattern_parse_adapter<V>,
        row: i64,
        col: i64,
    ) {
        adapter.handler.push((row, col, adapter.fwd_value.clone()));
    }

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:39`.
    pub fn handle_line_39<V: Clone + Default>(
        adapter: &mut pattern_parse_adapter<V>,
        row: i64,
        col: i64,
        val: V,
    ) {
        adapter.handler.push((row, col, val));
    }

    /// Translation of `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:43`.
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

    /// Translation of `complex_parse_adapter` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:67`.
    pub fn complex_parse_adapter_line_67<V: triplet_value_type>(
        handler: Vec<(i64, i64, V)>,
    ) -> complex_parse_adapter<V> {
        complex_parse_adapter { flags: 1, handler }
    }

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:69`.
    pub fn handle_line_69<V: triplet_value_type>(
        adapter: &mut complex_parse_adapter<V>,
        row: i64,
        col: i64,
    ) {
        adapter.handler.push((row, col, V::pattern_value()));
    }

    /// Translation of `handle` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:73`.
    pub fn handle_line_73<V: triplet_value_type>(
        adapter: &mut complex_parse_adapter<V>,
        row: i64,
        col: i64,
        real: f64,
    ) {
        let value = V::complex_value(real, 0.0).unwrap_or_else(|_| V::real_value(real));
        adapter.handler.push((row, col, value));
    }

    /// Translation of `get_chunk_handler` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:77`.
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

    /// Translation of `limit_parallelism_for_value_type` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:94`.
    pub fn limit_parallelism_for_value_type_line_94(_parallelism_selected: bool) -> bool {
        false
    }

    /// Translation of `limit_parallelism_for_value_type` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:99`.
    pub fn limit_parallelism_for_value_type_line_99(parallelism_selected: bool) -> bool {
        parallelism_selected
    }

    /// Translation of `get_symmetric_value` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:108`.
    pub fn get_symmetric_value_line_108<V: triplet_value_type>(
        value: &V,
        symmetry: symmetry_type,
    ) -> Result<V, fast_matrix_market::invalid_argument> {
        Ok(value.symmetric_value(symmetry))
    }

    /// Translation of `generalize_symmetry_coordinate` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:127`.
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

    /// Translation of `generalize_symmetry_array` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:165`.
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

    /// Translation of `read_real_or_complex` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:193`.
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

    /// Translation of `read_chunk_matrix_coordinate` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:213`.
    pub fn read_chunk_matrix_coordinate_line_213<V: triplet_value_type>(
        chunk: &str,
        header: &matrix_market_header,
        mut line: line_counts,
        options: &read_options,
    ) -> Result<(line_counts, Vec<(i64, i64, V)>), invalid_mm> {
        V::check_header_field(header.field)?;
        let mut entries = Vec::new();
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
            while pos < end && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r') {
                pos += 1;
            }
            if pos == end {
                line.file_line += bytes.iter().filter(|&&c| c == b'\n').count() as i64;
                continue;
            }
            if line.element_num >= header.nnz {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "Too many lines in file (file too long)".to_string(),
                    line.file_line + 1,
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
            while pos < end && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r') {
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
                while pos < end
                    && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r')
                {
                    pos += 1;
                }
                let real_start = pos;
                while pos < end
                    && bytes[pos] != b' '
                    && bytes[pos] != b'\t'
                    && bytes[pos] != b'\r'
                    && bytes[pos] != b'\n'
                {
                    pos += 1;
                }
                let real_field = &raw_line[real_start..pos];
                while pos < end
                    && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r')
                {
                    pos += 1;
                }
                let imaginary_start = pos;
                while pos < end
                    && bytes[pos] != b' '
                    && bytes[pos] != b'\t'
                    && bytes[pos] != b'\r'
                    && bytes[pos] != b'\n'
                {
                    pos += 1;
                }
                let imaginary_field = &raw_line[imaginary_start..pos];
                read_real_or_complex_line_193::<V>(&[real_field, imaginary_field], header, options)
            } else {
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
                let value_field = &raw_line[value_start..pos];
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

    /// Translation of `read_chunk_vector_coordinate` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:281`.
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

    /// Translation of `read_chunk_array` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:332`.
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

    /// Translation of `read_coordinate_body_sequential` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:412`.
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

    /// Translation of `read_array_body_sequential` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:436`.
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

    /// Translation of `read_matrix_market_body_no_adapters` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:459`.
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

        let threads = options.parallel_ok
            && options.num_threads != 1
            && !(header.symmetry != symmetry_type::general && header.format == format_type::array);

        let (lc, entries) = if threads {
            super::read_body_threads::read_body_threads_line_33::<V>(instream, header, options)?
        } else if header.format == format_type::coordinate {
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

    /// Translation of `read_matrix_market_body_no_pattern` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:525`.
    pub fn read_matrix_market_body_no_pattern_line_525<V: triplet_value_type>(
        instream: &mut impl BufRead,
        header: &matrix_market_header,
        options: &read_options,
    ) -> Result<Vec<(i64, i64, V)>, invalid_mm> {
        read_matrix_market_body_no_adapters_line_459::<V>(instream, header, options)
    }

    /// Translation of `read_matrix_market_body_no_pattern` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:543`.
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

    /// Translation of `read_matrix_market_body` at `fast_matrix_market/include/fast_matrix_market/read_body.hpp:562`.
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
    use super::thirdparty_task_thread_pool;
    use super::types::{matrix_market_header, read_options};
    use super::Placeholder;
    use std::collections::VecDeque;
    use std::io::BufRead;

    /// Original struct `line_count_result_s` at `fast_matrix_market/include/fast_matrix_market/read_body_threads.hpp:15`.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct line_count_result_s {
        /// Original C++ type: `std::string`.
        pub chunk: String,
        /// Original C++ type: `line_counts`.
        pub counts: line_counts,
    }

    /// Translation of `line_count_result_s` at `fast_matrix_market/include/fast_matrix_market/read_body_threads.hpp:19`.
    pub fn line_count_result_s_line_19(chunk: String, counts: line_counts) -> line_count_result_s {
        line_count_result_s { chunk, counts }
    }

    /// Translation of `count_chunk_lines` at `fast_matrix_market/include/fast_matrix_market/read_body_threads.hpp:24`.
    pub fn count_chunk_lines_line_24(chunk: String) -> line_count_result_s {
        let (file_line, empty_lines) = super::chunking::count_lines_line_66(&chunk);
        line_count_result_s_line_19(
            chunk,
            line_counts {
                file_line,
                element_num: file_line - empty_lines,
            },
        )
    }

    /// Translation of `read_body_threads` at `fast_matrix_market/include/fast_matrix_market/read_body_threads.hpp:33`.
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
        let mut parse_futures: VecDeque<
            std::sync::mpsc::Receiver<
                Result<(line_counts, Vec<(i64, i64, V)>), fast_matrix_market::invalid_mm>,
            >,
        > = VecDeque::new();
        let generalizing_symmetry_factor = if header.symmetry
            != super::types::symmetry_type::general
            && options.generalize_symmetry
        {
            2
        } else {
            1
        };
        let requested_threads = if options.num_threads < 1 {
            std::thread::available_parallelism()
                .map(|count| count.get())
                .unwrap_or(1)
        } else {
            options.num_threads as usize
        };
        let inflight_count = requested_threads.saturating_add(1).max(1);
        let mut pool = thirdparty_task_thread_pool::task_thread_pool_line_137(requested_threads);
        let mut entries = Vec::new();

        loop {
            while parse_futures.len() >= inflight_count {
                match parse_futures.pop_front().unwrap().recv().unwrap() {
                    Ok((_next_lc, mut next_entries)) => {
                        if generalizing_symmetry_factor > 1 {
                            entries.reserve(next_entries.len());
                        }
                        entries.append(&mut next_entries);
                    }
                    Err(err) => {
                        thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                        return Err(err);
                    }
                }
            }

            let chunk = super::chunking::get_next_chunk_line_51(instream, options)
                .map_err(|_| fast_matrix_market::invalid_mm_line_54("I/O error".to_string()))?;
            if chunk.is_empty() {
                break;
            }
            let counted = count_chunk_lines_line_24(chunk);
            if lc.element_num > header.nnz {
                return Err(fast_matrix_market::invalid_mm_line_55(
                    "File too long".to_string(),
                    lc.file_line + 1,
                ));
            }
            let start_lc = lc.clone();
            let counts = counted.counts;
            lc.file_line += counts.file_line;
            lc.element_num += counts.element_num;
            let header_for_task = header.clone();
            let options_for_task = options.clone();
            parse_futures.push_back(thirdparty_task_thread_pool::std_future_r_submit_line_248(
                &mut pool,
                move || {
                    if header_for_task.format == super::types::format_type::coordinate {
                        if header_for_task.object == super::types::object_type::matrix {
                            read_body::read_chunk_matrix_coordinate_line_213::<V>(
                                &counted.chunk,
                                &header_for_task,
                                start_lc,
                                &options_for_task,
                            )
                        } else {
                            read_body::read_chunk_vector_coordinate_line_281::<V>(
                                &counted.chunk,
                                &header_for_task,
                                start_lc,
                                &options_for_task,
                            )
                        }
                    } else {
                        let mut row = start_lc.element_num % header_for_task.nrows;
                        let mut col = start_lc.element_num / header_for_task.nrows;
                        read_body::read_chunk_array_line_332::<V>(
                            &counted.chunk,
                            &header_for_task,
                            start_lc,
                            &options_for_task,
                            &mut row,
                            &mut col,
                        )
                    }
                },
            ));
        }

        while let Some(rx) = parse_futures.pop_front() {
            match rx.recv().unwrap() {
                Ok((_next_lc, mut next_entries)) => {
                    if generalizing_symmetry_factor > 1 {
                        entries.reserve(next_entries.len());
                    }
                    entries.append(&mut next_entries);
                }
                Err(err) => {
                    thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
                    return Err(err);
                }
            }
        }
        thirdparty_task_thread_pool::task_thread_pool_line_149(&mut pool);
        Ok((lc, entries))
    }
}

pub mod thirdparty_task_thread_pool {
    use super::Placeholder;
    use std::collections::VecDeque;
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread::JoinHandle;

    type queued_task = Box<dyn FnOnce() + Send + 'static>;

    struct task_thread_pool_state {
        tasks: VecDeque<queued_task>,
        pool_running: bool,
        pool_paused: bool,
        notify_task_finish: bool,
        num_inflight_tasks: i64,
    }

    struct task_thread_pool_shared {
        state: Mutex<task_thread_pool_state>,
        task_cv: Condvar,
        task_finished_cv: Condvar,
    }

    /// Original class `task_thread_pool` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:130`.
    pub struct task_thread_pool {
        /// Original C++ type: `std::vector<std::thread>`.
        pub threads: Vec<Placeholder>,
        /// Original C++ type: `std::mutex`.
        pub thread_mutex: Placeholder,
        /// Original C++ type: `std::queue<std::packaged_task<void()>>`.
        pub tasks: Placeholder,
        pub queued_tasks: Vec<Box<dyn FnMut() + Send>>,
        worker_handles: Vec<JoinHandle<()>>,
        shared: Arc<task_thread_pool_shared>,
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

    /// Translation of `task_thread_pool` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:137`.
    pub fn task_thread_pool_line_137(num_threads: usize) -> task_thread_pool {
        let mut pool = task_thread_pool {
            threads: Vec::new(),
            thread_mutex: Placeholder,
            tasks: Placeholder,
            queued_tasks: Vec::new(),
            worker_handles: Vec::new(),
            shared: Arc::new(task_thread_pool_shared {
                state: Mutex::new(task_thread_pool_state {
                    tasks: VecDeque::new(),
                    pool_running: true,
                    pool_paused: false,
                    notify_task_finish: false,
                    num_inflight_tasks: 0,
                }),
                task_cv: Condvar::new(),
                task_finished_cv: Condvar::new(),
            }),
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

    /// Translation of `~task_thread_pool` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:149`.
    pub fn task_thread_pool_line_149(pool: &mut task_thread_pool) {
        unpause_line_218(pool);
        wait_for_tasks_line_291(pool);
        stop_all_threads_line_356(pool);
    }

    /// Translation of `clear_task_queue` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:160`.
    pub fn clear_task_queue_line_160(pool: &mut task_thread_pool) {
        pool.queued_tasks.clear();
        let mut state = pool.shared.state.lock().unwrap();
        state.tasks.clear();
        pool.shared.task_finished_cv.notify_all();
    }

    /// Translation of `get_num_queued_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:170`.
    pub fn get_num_queued_tasks_line_170(pool: &task_thread_pool) -> usize {
        pool.shared.state.lock().unwrap().tasks.len()
    }

    /// Translation of `get_num_running_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:180`.
    pub fn get_num_running_tasks_line_180(pool: &task_thread_pool) -> usize {
        pool.shared.state.lock().unwrap().num_inflight_tasks.max(0) as usize
    }

    /// Translation of `get_num_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:190`.
    pub fn get_num_tasks_line_190(pool: &task_thread_pool) -> usize {
        let state = pool.shared.state.lock().unwrap();
        state.tasks.len() + state.num_inflight_tasks.max(0) as usize
    }

    /// Translation of `get_num_threads` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:200`.
    pub fn get_num_threads_line_200(pool: &task_thread_pool) -> usize {
        pool.threads.len()
    }

    /// Translation of `pause` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:210`.
    pub fn pause_line_210(pool: &mut task_thread_pool) {
        let mut state = pool.shared.state.lock().unwrap();
        state.pool_paused = true;
        pool.pool_paused = true;
    }

    /// Translation of `unpause` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:218`.
    pub fn unpause_line_218(pool: &mut task_thread_pool) {
        let mut state = pool.shared.state.lock().unwrap();
        state.pool_paused = false;
        pool.pool_paused = false;
        pool.shared.task_cv.notify_all();
    }

    /// Translation of `is_paused` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:229`.
    pub fn is_paused_line_229(pool: &task_thread_pool) -> bool {
        pool.shared.state.lock().unwrap().pool_paused
    }

    /// Translation of `std::future<R> submit` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:248`.
    pub fn std_future_r_submit_line_248<F, R>(
        pool: &mut task_thread_pool,
        func: F,
    ) -> std::sync::mpsc::Receiver<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        submit_detach_line_260(pool, move || {
            let _ = tx.send(func());
        });
        rx
    }

    /// Translation of `submit_detach` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:260`.
    pub fn submit_detach_line_260<F>(pool: &mut task_thread_pool, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(func);
        let mut state = pool.shared.state.lock().unwrap();
        state.tasks.push_back(task);
        pool.shared.task_cv.notify_one();
    }

    /// Translation of `submit_detach` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:272`.
    pub fn submit_detach_line_272<F>(pool: &mut task_thread_pool, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        submit_detach_line_260(pool, func);
    }

    /// Translation of `wait_for_queued_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:281`.
    pub fn wait_for_queued_tasks_line_281(pool: &mut task_thread_pool) {
        let mut state = pool.shared.state.lock().unwrap();
        state.notify_task_finish = true;
        state = pool
            .shared
            .task_finished_cv
            .wait_while(state, |state| !state.tasks.is_empty())
            .unwrap();
        state.notify_task_finish = false;
    }

    /// Translation of `wait_for_tasks` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:291`.
    pub fn wait_for_tasks_line_291(pool: &mut task_thread_pool) {
        let mut state = pool.shared.state.lock().unwrap();
        state.notify_task_finish = true;
        state = pool
            .shared
            .task_finished_cv
            .wait_while(state, |state| {
                !state.tasks.is_empty() || state.num_inflight_tasks != 0
            })
            .unwrap();
        state.notify_task_finish = false;
    }

    /// Translation of `worker_main` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:303`.
    pub fn worker_main_line_303(pool: &mut task_thread_pool) {
        let shared = pool.shared.clone();
        loop {
            let task = {
                let mut state = shared.state.lock().unwrap();
                state = shared
                    .task_cv
                    .wait_while(state, |state| {
                        state.pool_running && (state.pool_paused || state.tasks.is_empty())
                    })
                    .unwrap();
                if !state.pool_running {
                    break;
                }
                let task = state.tasks.pop_front();
                if task.is_some() {
                    state.num_inflight_tasks += 1;
                    if state.notify_task_finish {
                        shared.task_finished_cv.notify_all();
                    }
                }
                task
            };

            if let Some(task) = task {
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(task));
                let mut state = shared.state.lock().unwrap();
                state.num_inflight_tasks -= 1;
                if state.notify_task_finish {
                    shared.task_finished_cv.notify_all();
                }
            }
        }
    }

    /// Translation of `start_threads` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:345`.
    pub fn start_threads_line_345(pool: &mut task_thread_pool, num_threads: usize) {
        let count = if num_threads < 1 {
            std::thread::available_parallelism()
                .map(|count| count.get())
                .unwrap_or(1)
        } else {
            num_threads
        };
        pool.threads = vec![Placeholder; count];
        for _ in 0..count {
            let shared = pool.shared.clone();
            pool.worker_handles.push(std::thread::spawn(move || loop {
                let task = {
                    let mut state = shared.state.lock().unwrap();
                    state = shared
                        .task_cv
                        .wait_while(state, |state| {
                            state.pool_running && (state.pool_paused || state.tasks.is_empty())
                        })
                        .unwrap();
                    if !state.pool_running {
                        break;
                    }
                    let task = state.tasks.pop_front();
                    if task.is_some() {
                        state.num_inflight_tasks += 1;
                        if state.notify_task_finish {
                            shared.task_finished_cv.notify_all();
                        }
                    }
                    task
                };

                if let Some(task) = task {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(task));
                    let mut state = shared.state.lock().unwrap();
                    state.num_inflight_tasks -= 1;
                    if state.notify_task_finish {
                        shared.task_finished_cv.notify_all();
                    }
                }
            }));
        }
    }

    /// Translation of `stop_all_threads` at `fast_matrix_market/include/fast_matrix_market/thirdparty/task_thread_pool.hpp:356`.
    pub fn stop_all_threads_line_356(pool: &mut task_thread_pool) {
        pool.pool_running = false;
        {
            let mut state = pool.shared.state.lock().unwrap();
            state.pool_running = false;
            pool.shared.task_cv.notify_all();
        }
        for handle in pool.worker_handles.drain(..) {
            let _ = handle.join();
        }
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

    /// Translation of `matrix_market_header` at `fast_matrix_market/include/fast_matrix_market/types.hpp:48`.
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

    /// Translation of `matrix_market_header` at `fast_matrix_market/include/fast_matrix_market/types.hpp:49`.
    pub fn matrix_market_header_line_49(vector_length: i64) -> matrix_market_header {
        let mut header = matrix_market_header_line_48();
        header.object = object_type::vector;
        header.vector_length = vector_length;
        header
    }

    /// Translation of `matrix_market_header` at `fast_matrix_market/include/fast_matrix_market/types.hpp:50`.
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

    /// Translation of `get_field_type` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:16`.
    pub fn get_field_type_line_16<T>(_type: *const T) -> field_type {
        field_type::integer
    }

    /// Translation of `get_field_type` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:24`.
    pub fn get_field_type_line_24<T>(_type: *const T) -> field_type {
        field_type::real
    }

    /// Translation of `get_field_type` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:32`.
    pub fn get_field_type_line_32<T>(_type: *const T) -> field_type {
        field_type::complex
    }

    /// Translation of `get_field_type` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:40`.
    pub fn get_field_type_line_40(_type: *const pattern_placeholder_type) -> field_type {
        field_type::pattern
    }

    /// Translation of `write_body_sequential` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:50`.
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

    /// Translation of `write_body` at `fast_matrix_market/include/fast_matrix_market/write_body.hpp:66`.
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

    /// Translation of `write_body_threads` at `fast_matrix_market/include/fast_matrix_market/write_body_threads.hpp:20`.
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
