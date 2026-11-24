fn issue14164() -> Result<u32, ()> {
    let v = std::cell::RefCell::new(Some(vec![1]));
    let r = match &*v.borrow() {
        Some(v) => Ok(Ok(v[0])),
        None => Ok(Ok(0)),
    }?;
    r
    //~[edition2024]^ let_and_return
}

fn issue15987() -> i32 {
    macro_rules! sample {
        ( $( $args:expr ),+ ) => {};
    }

    let r = 5;
    sample!(r);
    r
}

fn has_comment() -> Vec<usize> {
    let v = Vec::new();

    // TODO: stuff

    v
}

mod issue16451 {

    fn expect_on_return_expr() -> std::collections::BTreeMap<(), ()> {
        let stuff = std::collections::BTreeMap::new();

        // TODO: Fill `stuff`

        #[expect(clippy::let_and_return)]
        stuff
    }

    fn cfg_on_return_expr() -> i32 {
        let x = 5;
        #[cfg(debug_assertions)]
        x
    }
}

fn wrongly_unmangled_macros() -> i32 {
    let x = 1;
    macro_rules! plus_one {
        ($e:expr) => {
            $e + 1
        };
    }

    let y = plus_one!(x);
    y
    //~^ let_and_return
}

fn issue16820() -> Option<i32> {
    let value = Some(42);

    let v @ None = value else {
        panic!("uh oh!");
    };

    v
}
