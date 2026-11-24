fn main() {
    let _ = Some(0).filter(|_| match 0 {
        //~^ some_filter
        0 => false,
        1 => true,
        _ => true,
    });
}
