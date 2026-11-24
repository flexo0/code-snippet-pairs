fn main() {
    let _ = Some(5).map_or(false, |n| {
        //~^ unnecessary_map_or
        let _ = 1;
        n == 5
    });
    let _ = Some(5).map_or(false, |n| {
        //~^ unnecessary_map_or
        let _ = n;
        6 >= 5
    });
}
