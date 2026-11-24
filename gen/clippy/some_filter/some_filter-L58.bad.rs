fn main() {
    Some(String::from(
        //~^ some_filter
        "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong",
    ))
    .filter(|_| 1 == 0);
    Some(5).filter(|_| {
        //~^ some_filter
        "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong".is_empty()
    });
    Some(String::from(
        //~^ some_filter
        "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong",
    ))
    .filter(|_| {
        "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong".is_empty()
    });
}
