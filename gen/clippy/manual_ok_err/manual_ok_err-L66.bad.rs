fn main() {
    let _ = match -S {
        //~^ manual_ok_err
        Ok(v) => Some(v),
        _ => None,
    };
}
