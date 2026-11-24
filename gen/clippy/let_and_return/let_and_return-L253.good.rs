fn issue14164() -> Result<u32, ()> {
    let v = std::cell::RefCell::new(Some(vec![1]));
    let r = match &*v.borrow() {
        Some(v) => Ok(Ok(v[0])),
        None => Ok(Ok(0)),
    }?;
    r
}
