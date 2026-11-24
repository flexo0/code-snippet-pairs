mod issue15051 {
    fn with_addr_of(x: &Container) -> Option<&bool> {
        (&x.field).as_ref().ok()
    }
}
