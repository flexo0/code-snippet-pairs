fn main() {
    let _lint_negated = if x {
        //~^ manual_take
        x = false;
        false
    } else {
        true
    };
}
