fn main() {
    let _ = vec![Ok(1) as Result<i32, ()>]
        .into_iter()
        .filter(Result::is_ok)
        //~^ result_filter_map
        .map(Result::unwrap);
}
