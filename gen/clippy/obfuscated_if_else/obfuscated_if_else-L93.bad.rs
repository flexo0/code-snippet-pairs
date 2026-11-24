fn issue16288() {
    true.then(|| format!("this is a test")).unwrap_or_default();
}
