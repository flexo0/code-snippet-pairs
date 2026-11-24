fn main() {
    let _ = Some(5) == Some(5);
    let _ = Some(5).is_some_and(|n| {
        //~^ unnecessary_map_or
        let _ = n;
        6 >= 5
    });
}
