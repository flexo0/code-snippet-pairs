fn simple_equal() {
    iter::<Option<&u8>>().find_map(|x| x.cloned());
}
