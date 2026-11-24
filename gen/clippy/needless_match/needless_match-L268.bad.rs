mod issue9084 {
    fn wildcard_if() {
        let _ = match e {
            //~^ needless_match
            Some(i) => Some(i),
            _ if some_bool => e,
            _ => e,
        };
    }
}
