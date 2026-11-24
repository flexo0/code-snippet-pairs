fn main() {
    let _: Option<i32> = opt.map_or(None, |x| {
    //~^ option_map_or_none
                        Some(x + 1)
                       });
    // function returning `Option`
    let _: Option<i32> = opt.map_or(None, bar);
    //~^ option_map_or_none
    let _: Option<i32> = opt.map_or(None, |x| {
        //~^ option_map_or_none
        let offset = 0;
        let height = x;
        Some(offset + height)
    });
}
