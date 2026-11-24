fn simple_equal() {
    iter::<Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());
}
