fn by_ref(vals: &mut Vec<u32>) {
    let _ = vals.as_slice().len();
}
