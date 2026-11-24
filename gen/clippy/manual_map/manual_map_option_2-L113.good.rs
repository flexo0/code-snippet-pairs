mod with_type_coercion {
    fn foo<T: DummyTrait, F: Fn() -> Result<T, ()>>(f: F) {
        let _ = Some(0).map(|_| match f() {
                Ok(res) => Ok(Box::new(res)),
                _ => Err(()),
            });
    }
}
