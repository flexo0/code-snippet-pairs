pub fn manual_mem_take_should_reference_core() {
    let _lint = core::mem::take(&mut x);
}
