fn main() {
    (1 == 0).then_some(String::from(
        //~^ some_filter
        "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong",
    ));
    {
        //~^ some_filter
        "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong".is_empty()
    }.then_some(5);
    "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong".is_empty().then_some(String::from(
        //~^ some_filter
        "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong",
    ));
}
