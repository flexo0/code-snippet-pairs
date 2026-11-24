fn issue10000() {
    fn add_turbofish_necessary() -> impl Add {
        (0..3).fold(0, |acc, x| acc + x)
        //~^ unnecessary_fold
    }
}
