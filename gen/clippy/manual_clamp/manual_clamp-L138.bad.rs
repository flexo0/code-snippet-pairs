fn const_main() {
    let x0 = if CONST_MAX < input {
        //~^ manual_clamp

        CONST_MAX
    } else if CONST_MIN > input {
        CONST_MIN
    } else {
        input
    };
}
