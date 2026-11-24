fn const_main() {
    if x9 < CONST_MIN {
        //~^ manual_clamp

        x9 = CONST_MIN;
    }
    if x9 > CONST_MAX {
        x9 = CONST_MAX;
    }

    let x10 = match input {
        //~^ manual_clamp
        x if CONST_MIN > x => CONST_MIN,
        x if CONST_MAX < x => CONST_MAX,
        x => x,
    };
}
