fn result_unwrap_or() {
    match a {
        //~^ manual_unwrap_or
        Ok(i) => i,
        Err(_) => 42,
    };
}
