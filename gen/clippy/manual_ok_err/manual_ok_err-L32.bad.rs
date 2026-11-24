fn main() {
    let _ = if let Ok(v) = funcall() {
        //~^ manual_ok_err

        Some(v)
    } else {
        None
    };

    let _ = if let Err(v) = funcall() {
        //~^ manual_ok_err

        Some(v)
    } else {
        None
    };
}
