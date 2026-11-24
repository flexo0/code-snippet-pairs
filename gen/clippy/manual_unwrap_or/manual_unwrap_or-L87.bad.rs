fn option_unwrap_or() {
    if let Some(x) = Some(1) {
        //~^ manual_unwrap_or
        x
    } else {
        42
    };
}
