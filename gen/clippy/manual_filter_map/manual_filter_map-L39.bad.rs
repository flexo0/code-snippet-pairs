fn simple_equal() {
    iter::<Option<&u8>>().find(|x| x.is_some()).map(|x| x.cloned().unwrap());
}
