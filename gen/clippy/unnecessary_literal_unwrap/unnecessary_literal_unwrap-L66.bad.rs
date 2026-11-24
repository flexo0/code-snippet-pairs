fn unwrap_result_ok() {
    Ok::<_, ()>(1).expect("this never happens");
}
