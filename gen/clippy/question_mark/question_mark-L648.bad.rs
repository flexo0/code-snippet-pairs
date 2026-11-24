fn issue16429(b: i32) -> Option<i32> {
    let _ = if b == 1 {
        b
    } else if let Some(x) = a {
        //~^ question_mark
        x
    } else {
        return None;
    };
}
