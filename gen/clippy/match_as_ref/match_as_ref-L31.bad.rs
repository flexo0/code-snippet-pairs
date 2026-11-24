mod issue4437 {
    impl Error for E {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self.source {
                //~^ match_as_ref
                Some(ref s) => Some(s),
                None => None,
            }
        }
    }
}
