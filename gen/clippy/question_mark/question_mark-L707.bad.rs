fn issue16751(mut v: Option<usize>) -> Option<usize> {
    match v {
        //~^ question_mark
        Some(n) => if n > 10 { Some(42) } else { None },
        None => return None,
    }
}
