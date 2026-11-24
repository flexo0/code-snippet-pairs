fn issue10000() {
    fn smoketest_map<S: BuildHasher>(mut map: HashMap<i32, i32, S>) {
        let _ = map.values().sum::<i32>();
    }
}
