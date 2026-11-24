mod with_type_coercion {
    fn foo<T: DummyTrait, F: Fn() -> Result<T, ()>>(f: F) {
        let _ = match Some(0) {
            //~^ manual_map
            Some(_) => Some(match f() {
                Ok(res) => Ok(Box::new(res)),
                _ => Err(()),
            }),
            None => None,
        };
    }
}
