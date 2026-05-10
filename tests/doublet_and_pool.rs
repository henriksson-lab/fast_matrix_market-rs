use fast_matrix_market::app_doublet::{
    read_matrix_market_doublet_line_66, write_matrix_market_doublet_line_79,
};
use fast_matrix_market::thirdparty_task_thread_pool::{
    clear_task_queue_line_160, get_num_queued_tasks_line_170, get_num_tasks_line_190,
    get_num_threads_line_200, is_paused_line_229, pause_line_210, submit_detach_line_260,
    task_thread_pool_line_137, task_thread_pool_line_149, unpause_line_218,
    wait_for_tasks_line_291,
};
use fast_matrix_market::types::{
    field_type, generalize_coordinate_diagnonal_values_type, matrix_market_header_line_49,
    out_of_range_behavior, read_options, write_options,
};
use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

#[test]
fn doublet_binding_reads_vector_fixture() {
    let mut input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/vector_coordinate.mtx"
    ));

    let (length, indices, values) =
        read_matrix_market_doublet_line_66::<Vec<f64>>(&mut input, &read_opts()).unwrap();

    assert_eq!(length, 4);
    assert_eq!(indices, vec![0, 1, 3]);
    assert_eq!(values, vec![vec![101.0], vec![202.0], vec![404.0]]);
}

#[test]
fn doublet_binding_reads_template_value_types() {
    let mut complex_input = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/vector_coordinate_complex.mtx"
    ));
    let (_length, _indices, values) =
        read_matrix_market_doublet_line_66::<(f64, f64)>(&mut complex_input, &read_opts()).unwrap();
    assert!(values.iter().any(|value| value.1 != 0.0));

    let mut complex_as_real = Cursor::new(include_str!(
        "../fast_matrix_market/tests/matrices/vector_coordinate_complex.mtx"
    ));
    let err =
        read_matrix_market_doublet_line_66::<f64>(&mut complex_as_real, &read_opts()).unwrap_err();
    assert_eq!(
        err.msg,
        "Matrix Market file has complex fields but passed data structure cannot handle complex values."
    );
}

#[test]
fn doublet_binding_writes_vector_coordinate_text() {
    let mut header = matrix_market_header_line_49(3);
    header.field = field_type::real;
    let mut out = Vec::new();

    write_matrix_market_doublet_line_79(
        &mut out,
        header,
        vec![0, 2],
        vec![vec![1.0], vec![3.0]],
        &write_opts(),
    )
    .unwrap();

    assert_eq!(
        String::from_utf8(out).unwrap(),
        "%%MatrixMarket vector coordinate real general\n3 2\n1 1\n3 3\n"
    );
}

#[test]
fn doublet_binding_writes_template_value_types() {
    let header = matrix_market_header_line_49(3);

    let mut real_out = Vec::new();
    write_matrix_market_doublet_line_79(
        &mut real_out,
        header.clone(),
        vec![0, 2],
        vec![1.5_f64, 3.25],
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(real_out).unwrap(),
        "%%MatrixMarket vector coordinate real general\n3 2\n1 1.5\n3 3.25\n"
    );

    let mut complex_out = Vec::new();
    write_matrix_market_doublet_line_79(
        &mut complex_out,
        header.clone(),
        vec![1],
        vec![(2.0_f64, -3.5_f64)],
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(complex_out).unwrap(),
        "%%MatrixMarket vector coordinate complex general\n3 1\n2 2 -3.5\n"
    );

    let mut pattern_header = header;
    pattern_header.field = field_type::pattern;
    let mut pattern_out = Vec::new();
    write_matrix_market_doublet_line_79(
        &mut pattern_out,
        pattern_header,
        vec![0, 2],
        vec![(), ()],
        &write_opts(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(pattern_out).unwrap(),
        "%%MatrixMarket vector coordinate pattern general\n3 2\n1\n3\n"
    );
}

#[test]
fn task_pool_queue_runs_detached_tasks_when_waiting() {
    let mut pool = task_thread_pool_line_137(2);
    assert_eq!(get_num_threads_line_200(&pool), 2);
    pause_line_210(&mut pool);

    let value = Arc::new(Mutex::new(0));
    let value_for_task = value.clone();
    submit_detach_line_260(&mut pool, move || {
        *value_for_task.lock().unwrap() += 1;
    });
    assert_eq!(get_num_queued_tasks_line_170(&pool), 1);

    unpause_line_218(&mut pool);
    wait_for_tasks_line_291(&mut pool);

    assert_eq!(*value.lock().unwrap(), 1);
    assert_eq!(get_num_queued_tasks_line_170(&pool), 0);

    pause_line_210(&mut pool);
    assert!(is_paused_line_229(&pool));
    unpause_line_218(&mut pool);
    assert!(!is_paused_line_229(&pool));
    clear_task_queue_line_160(&mut pool);
    task_thread_pool_line_149(&mut pool);
    assert_eq!(get_num_threads_line_200(&pool), 0);
}

#[test]
fn task_pool_runs_tasks_on_workers() {
    let mut pool = task_thread_pool_line_137(2);
    let current = Arc::new(AtomicUsize::new(0));
    let max_seen = Arc::new(AtomicUsize::new(0));

    for _ in 0..2 {
        let current = current.clone();
        let max_seen = max_seen.clone();
        submit_detach_line_260(&mut pool, move || {
            let now = current.fetch_add(1, Ordering::SeqCst) + 1;
            max_seen.fetch_max(now, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(30));
            current.fetch_sub(1, Ordering::SeqCst);
        });
    }

    wait_for_tasks_line_291(&mut pool);
    assert_eq!(get_num_tasks_line_190(&pool), 0);
    assert!(max_seen.load(Ordering::SeqCst) >= 2);
    task_thread_pool_line_149(&mut pool);
}
