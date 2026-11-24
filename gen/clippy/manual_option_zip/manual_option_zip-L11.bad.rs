fn should_lint() {
    let _ = a.and_then(|a| b.map(|b| (a, b)));
}
