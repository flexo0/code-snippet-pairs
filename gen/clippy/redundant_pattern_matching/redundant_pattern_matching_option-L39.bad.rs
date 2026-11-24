fn issue_17286(a: bool, b: bool, opt: Option<i32>) {
    if matches!(opt, None if a) {}
}
