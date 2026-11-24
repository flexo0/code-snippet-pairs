fn main() {
    let _ = vec![Ok(1) as Result<i32, ()>]
        .into_iter()
        .flatten();
}
