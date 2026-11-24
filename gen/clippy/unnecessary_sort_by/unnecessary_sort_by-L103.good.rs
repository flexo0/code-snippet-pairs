mod issue_6001 {
    pub fn test() {
        args.sort_by_key(|a| std::cmp::Reverse(a.name()));
    }
}
