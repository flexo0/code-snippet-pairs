fn issue_destructuring_assignment() -> Option<(i32, i32)> {
    {
        let x = opt?;
        (a, b) = x
    }
}
