fn const_main() {
    if CONST_MAX < x13 {
        //~^ manual_clamp

        x13 = CONST_MAX;
    }
    if CONST_MIN > x13 {
        x13 = CONST_MIN;
    }

    let x14 = if input > CONST_MAX {
        //~^ manual_clamp

        CONST_MAX
    } else if input < CONST_MIN {
        CONST_MIN
    } else {
        input
    };
}
