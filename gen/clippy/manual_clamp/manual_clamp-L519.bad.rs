fn msrv_1_50() {
    let _ = if input > CONST_MAX {
        //~^ manual_clamp

        CONST_MAX
    } else if input < CONST_MIN {
        CONST_MIN
    } else {
        input
    };
}
