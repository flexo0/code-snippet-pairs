fn const_main() {
    let x2 = if input < CONST_MIN {
        //~^ manual_clamp

        CONST_MIN
    } else if input > CONST_MAX {
        CONST_MAX
    } else {
        input
    };

    let x3 = if CONST_MIN > input {
        //~^ manual_clamp

        CONST_MIN
    } else if CONST_MAX < input {
        CONST_MAX
    } else {
        input
    };

    let x4 = input.max(CONST_MIN).min(CONST_MAX);
}
