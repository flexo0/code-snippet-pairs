fn issue16330() {
    let a;
    let b;
    if true {
        //~^ needless_late_init
        a = 1;
        b = 2;
    } else {
        a = 3;
        b = 4;
    }
}
