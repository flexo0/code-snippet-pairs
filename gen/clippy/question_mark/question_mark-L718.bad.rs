fn issue_destructuring_assignment() -> Option<(i32, i32)> {
    match opt {
        //~^ question_mark
        Some(x) => (a, b) = x,
        None => return None,
    }
}
