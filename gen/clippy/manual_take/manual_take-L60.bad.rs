fn msrv_1_40() -> bool {
    if x {
        //~^ manual_take
        x = false;
        true
    } else {
        false
    }
}
