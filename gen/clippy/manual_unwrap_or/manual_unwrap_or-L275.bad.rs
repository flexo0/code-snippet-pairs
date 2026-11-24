mod issue6965 {
    fn test() {
        let _ = match some_macro!() {
            //~^ manual_unwrap_or
            Some(val) => val,
            None => 0,
        };
    }
}
