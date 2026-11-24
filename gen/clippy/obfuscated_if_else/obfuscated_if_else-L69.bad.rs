fn issue11141() {
    let _ = true.then_some(30).unwrap_or(17) | true.then_some(2).unwrap_or(3) | true.then_some(10).unwrap_or(1);
}
