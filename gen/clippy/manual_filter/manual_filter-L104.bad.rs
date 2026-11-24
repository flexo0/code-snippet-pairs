fn main() {
    if let Some(x) = Some(7) {
        //~^ manual_filter
        if external_cond { Some(x) } else { None }
    } else {
        None
    };
}
