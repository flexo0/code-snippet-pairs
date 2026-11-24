fn simple_equal() {
    iter::<Result<&String, ()>>().find_map(|y| to_ref(y).cloned().ok());
}
