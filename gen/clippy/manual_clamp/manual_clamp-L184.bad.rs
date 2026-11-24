fn const_main() {
    let x6 = match input {
        //~^ manual_clamp
        x if x > CONST_MAX => CONST_MAX,
        x if x < CONST_MIN => CONST_MIN,
        x => x,
    };
}
