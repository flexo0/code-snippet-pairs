fn simple_equal() {
    iter::<Result<u8, ()>>().find_map(|x| x.ok());
}
