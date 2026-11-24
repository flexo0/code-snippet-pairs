fn main() {
    let _ = match funcall() {
        //~^ manual_ok_err
        Ok(v) => Some(v),
        _v => None,
    };
}
