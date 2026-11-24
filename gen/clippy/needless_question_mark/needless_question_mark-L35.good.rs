fn simple_option_bad4(to: Option<TO>) -> Option<usize> {
    to.and_then(|t| t.magic)
    //~^ needless_question_mark
}
