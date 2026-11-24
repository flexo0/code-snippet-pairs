fn main() {
    let _ = vec![Ok(1) as Result<i32, ()>]
        .into_iter()
        .filter(|o| o.is_ok())
        //~^ result_filter_map
        .map(|o| o.unwrap());
}
