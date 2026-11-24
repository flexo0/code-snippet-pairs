async fn async_wrapped<T>(a: Option<T>) -> Option<T> {
    { a }
}
