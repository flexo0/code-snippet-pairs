fn issue16751(mut v: Option<usize>) -> Option<usize> {
    let _ = if let Some(ref mut n) = v {
        //~^ question_mark
        println!("{n}");
        42
    } else {
        return None;
    };
}
