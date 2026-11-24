mod issue4437 {
    impl Error for E {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source.as_ref().map(|x| x as _)
        }
    }
}
