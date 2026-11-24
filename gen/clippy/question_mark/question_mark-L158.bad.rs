fn func() -> Option<i32> {
    let _val = match f() {
        //~^ question_mark
        Some(val) => val,
        None => return None,
    };
}
