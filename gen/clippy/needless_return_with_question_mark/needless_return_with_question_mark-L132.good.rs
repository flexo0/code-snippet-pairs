async fn async_fn() -> Result<(), ()> {
    if true {
        Err(())?;
        //~^ needless_return_with_question_mark
    }
}
