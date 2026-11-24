fn issue11141() {
    let _ = *true.then_some(&42).unwrap_or(&17);
}
