mod issue_6001 {
    pub fn test() {
        args.sort_by(|a, b| b.name().cmp(&a.name()));
    }
}
