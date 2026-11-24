fn issue16330() {
    let (a, b) = if true {
        //~^ needless_late_init
        (1, 2)
    } else {
        (3, 4)
    };
}
