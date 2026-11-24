fn unwrap_or_else_default() {
    empty_string.unwrap_or_else(|| "".to_string());
}
