fn simple_equal() {
    iter::<Result<&String, ()>>().find(|&x| to_ref(x).is_ok()).map(|y| to_ref(y).cloned().unwrap());
}
