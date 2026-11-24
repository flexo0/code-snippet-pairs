fn main() {
    let _ = result.map_or_else(|_: i32| false, |_: i32| true);
}
