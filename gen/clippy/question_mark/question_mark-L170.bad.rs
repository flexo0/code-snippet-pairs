fn func() -> Option<i32> {
    match f() {
        //~^ question_mark
        Some(val) => val,
        None => return None,
    };
}
