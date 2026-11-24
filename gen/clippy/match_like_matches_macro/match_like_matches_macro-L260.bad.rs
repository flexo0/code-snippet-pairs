fn msrv_1_42() {
    let _y = match Some(5) {
        Some(0) => true,
        _ => false,
    };
}
