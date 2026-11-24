macro_rules! y {
    () => {
        let x = 33u32;
        let _ = x.div_ceil(8);
        //~^ manual_div_ceil
        let _ = x.div_ceil(8);
        //~^ manual_div_ceil
    };
}
