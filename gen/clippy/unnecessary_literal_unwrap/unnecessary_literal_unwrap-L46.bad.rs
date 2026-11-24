fn unwrap_option_none() {
    None::<u16>.unwrap_or_else(|| 234);
}
