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
        {
            { a };
            ()
        },
        (),
    );

    let y = [
        { todo!() },
        {
            { a };
            ()
        },
        (),
    ];

    fn call(x: (), y: (), z: ()) {}
    let y = call(
        { todo!() },
        {
            { a };
            ()
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
        {
            { a };
            ()
        },
        (),
    );

    -{
        { a };
        1
    };

    _ = { a };
    1;

    if let x = {
        { a };
        1
    } {}

    if {
        { a };
        true
    } {
        todo!()
    }

    [1, 2, 3][{
        { a };
        1usize
    }];

    todo!()
}
