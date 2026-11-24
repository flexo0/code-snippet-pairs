fn issue16330() {
    let (a, b, c) = if true {
        //~^ needless_late_init
        (1, 2, 3)
    } else if false {
        (4, 5, 6)
    } else {
        (7, 8, 9)
    };
}
