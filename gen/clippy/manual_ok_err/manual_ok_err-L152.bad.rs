mod issue15051 {
    fn with_addr_of(x: &Container) -> Option<&bool> {
        match &x.field {
            //~^ manual_ok_err
            Ok(panel) => Some(panel),
            Err(_) => None,
        }
    }
}
