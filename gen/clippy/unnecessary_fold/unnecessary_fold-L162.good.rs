fn issue10000() {
    fn add_turbofish_necessary() -> impl Add {
        (0..3).sum::<i32>()
        //~^ unnecessary_fold
    }
}
