fn unwrap_option_none() {
    let _val: u16 = None.unwrap_or_else(|| { 234 });
}
