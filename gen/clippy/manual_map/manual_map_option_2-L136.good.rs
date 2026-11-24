mod with_type_coercion {
    fn bar() {
        let _: Option<Box<&[u8]>> = Some(0).map(|_| g(x));
    }
}
