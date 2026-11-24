fn unwrap_result_err() {
    Err::<(), _>(1).expect("this always happens");
}
