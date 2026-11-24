fn issue16751(mut v: Option<usize>) -> Option<usize> {
    let _ = {
        let ref mut n = v?;
        //~^ question_mark
        println!("{n}");
        42
    };
}
