fn msrv_1_13(arg: Option<i32>) -> Option<i32> {
    if arg.is_none() {
        //~^ question_mark
        return None;
    }
    let val = match arg {
        //~^ question_mark
        Some(val) => val,
        None => return None,
    };
}
