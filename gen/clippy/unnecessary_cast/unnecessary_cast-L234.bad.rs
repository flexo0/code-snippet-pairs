mod fixable {
    fn issue_9380() {
        let _: i32 = -(1) as i32;
    }
}
