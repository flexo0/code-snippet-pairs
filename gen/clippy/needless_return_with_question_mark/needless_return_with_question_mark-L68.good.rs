fn issue11616() -> Result<(), ()> {
    let _x: () = {
        Err(())?;
        //~^ ERROR: unneeded `return` statement with `?` operator
    };
}
