fn main() {
    let _ = match 0 {
        //~^ some_filter
        0 => false,
        1 => true,
        _ => true,
    }.then_some(0);
}
