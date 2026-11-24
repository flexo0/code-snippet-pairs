pub fn manual_mem_take_should_reference_core() {
    let _lint = if x {
        //~^ manual_take
        x = false;
        true
    } else {
        false
    };
}
