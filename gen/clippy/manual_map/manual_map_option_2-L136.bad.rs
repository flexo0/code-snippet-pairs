mod with_type_coercion {
    fn bar() {
        let _: Option<Box<&[u8]>> = match Some(0) {
            //~^ manual_map
            Some(_) => Some(g(x)),
            None => None,
        };
    }
}
