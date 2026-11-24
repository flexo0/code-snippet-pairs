fn main() {
    let _ = if (a + b) > (c + d) {
        //~^ manual_abs_diff
        (a + b) - (c + d)
    } else {
        (c + d) - (a + b)
    };
    let _ = if (a + b) < (c + d) {
        //~^ manual_abs_diff
        (c + d) - (a + b)
    } else {
        (a + b) - (c + d)
    };
}
