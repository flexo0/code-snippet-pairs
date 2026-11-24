macro_rules! y {
    () => {
        let x = 33u32;
        let _ = (x + 7) / 8;
        //~^ manual_div_ceil
        let _ = (7 + x) / 8;
        //~^ manual_div_ceil
    };
}
