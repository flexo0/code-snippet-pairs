fn issue16288() {
    if true { format!("this is a test") } else { Default::default() };
}
