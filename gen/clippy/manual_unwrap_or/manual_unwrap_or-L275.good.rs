mod issue6965 {
    fn test() {
        let _ = some_macro!().unwrap_or(0);
    }
}
