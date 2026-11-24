mod issue14577 {
    trait Unit {}
    impl Unit for () {}

    fn bar() -> () {
        //~[edition2021]^ unused_unit
        panic!()
    }

    struct UnitStruct;
    impl UnitStruct {
        fn apply<F: for<'c> Fn(&'c mut Self)>(&mut self, f: F) {
            todo!()
        }
    }
}

mod pr14962 {
    type UnusedParensButNoUnit = Box<dyn (Fn())>;
}


mod issue15035 {

    trait Convert<T> {
        fn from(value: T) -> Self;
    }

    impl Convert<u64> for () {
        fn from(_value: u64) -> Self {}
    }

    fn handle<T: Convert<u64>>(value: u64) -> T {
        Convert::from(value)
    }

    pub fn f() -> Option<bool> {
        let result: Result<bool, u64> = Err(42);
        // the `-> ()` is required for the inference of `handle`'s return type
        result.map_err(|err| -> () { handle(err) }).ok()
    }
}
