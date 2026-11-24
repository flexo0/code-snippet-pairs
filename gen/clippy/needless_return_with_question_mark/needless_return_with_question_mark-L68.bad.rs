fn issue11616() -> Result<(), ()> {
    let _x: () = {
        return Err(())?;
        //~^ ERROR: unneeded `return` statement with `?` operator
    };
}
