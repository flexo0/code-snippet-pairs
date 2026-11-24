fn issue10000() {
    fn add_turbofish_not_necessary() -> i32 {
        (0..3).fold(0, |acc, x| acc + x)
        //~^ unnecessary_fold
    }
}
