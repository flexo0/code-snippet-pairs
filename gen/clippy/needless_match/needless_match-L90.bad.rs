fn if_let_option() {
    let _ = if let Some(a) = Some(1) { Some(a) } else { None };
}
