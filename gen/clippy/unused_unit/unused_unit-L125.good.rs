mod issue14577 {
    trait Unit {}
    impl Unit for () {}

    fn run<R: Unit>(f: impl FnOnce() -> R) {
        f();
    }

    fn bar() {
        run(|| -> () { todo!() });
    }

    struct UnitStruct;
    impl UnitStruct {
        fn apply<F: for<'c> Fn(&'c mut Self)>(&mut self, f: F) {
            todo!()
        }
    }
}
