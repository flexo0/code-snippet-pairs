fn issue8817() {
    Some(5)
        .map(f)
        //~^ redundant_closure
        //~| HELP: replace the closure with the function itself
        .map(g)
        //~^ redundant_closure
        //~| HELP: replace the closure with the function itself
        .map(S)
        //~^ redundant_closure
        //~| HELP: replace the closure with the tuple struct itself
        .map(MyError::A)
        //~^ redundant_closure
        //~| HELP: replace the closure with the tuple variant itself
        .unwrap(); // just for nicer formatting
}
