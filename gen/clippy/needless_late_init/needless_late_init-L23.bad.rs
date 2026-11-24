fn simple() {
    let b;
    //~^ needless_late_init
    let c;
    //~^ needless_late_init
    b = 1;
    c = 2;
}
