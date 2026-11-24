fn issue16330() {
    let a;
    let b;
    let c;
    if true {
        //~^ needless_late_init
        a = 1;
        b = 2;
        c = 3;
    } else if false {
        a = 4;
        b = 5;
        c = 6;
    } else {
        a = 7;
        b = 8;
        c = 9;
    }
}
