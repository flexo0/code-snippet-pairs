fn main() {
    let _: Option<i32> = opt.map(|x| x + 1);
    // function returning `Option`
    let _: Option<i32> = opt.and_then(bar);
    //~^ option_map_or_none
    let _: Option<i32> = opt.and_then(|x| {
        //~^ option_map_or_none
        let offset = 0;
        let height = x;
        Some(offset + height)
    });
}
