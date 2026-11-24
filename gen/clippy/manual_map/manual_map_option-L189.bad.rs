fn main() {
    match option_env!("") {
        //~^ manual_map
        Some(x) => Some(String::from(x)),
        None => None,
    };
}
