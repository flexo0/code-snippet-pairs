#[allow(
    irrefutable_let_patterns,
    clippy::blocks_in_conditions,
    clippy::unused_unit,
    clippy::let_unit_value,
    clippy::unit_arg,
    clippy::unnecessary_operation
)]
fn issue15537(a: i32) -> ((), (), ()) {
    let y = (
        { todo!() },
        match { a } {
            //~^ match_single_binding
            _ => (),
        },
        (),
    );

    let y = [
        { todo!() },
        match { a } {
            //~^ match_single_binding
            _ => (),
        },
        (),
    ];

    fn call(x: (), y: (), z: ()) {}
    let y = call(
        { todo!() },
        match { a } {
            //~^ match_single_binding
            _ => (),
        },
        (),
    );

    struct Foo;
    impl Foo {
        fn method(&self, x: (), y: (), z: ()) {}
    }
    let x = Foo;
    x.method(
        { todo!() },
        match { a } {
            //~^ match_single_binding
            _ => (),
        },
        (),
    );

    -match { a } {
        //~^ match_single_binding
        _ => 1,
    };

    _ = match { a } {
        //~^ match_single_binding
        _ => 1,
    };

    if let x = match { a } {
        //~^ match_single_binding
        _ => 1,
    } {}

    if match { a } {
        //~^ match_single_binding
        _ => true,
    } {
        todo!()
    }

    [1, 2, 3][match { a } {
        //~^ match_single_binding
        _ => 1usize,
    }];

    todo!()
}
