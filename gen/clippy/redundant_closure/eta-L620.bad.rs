fn issue8817() {
    Some(5)
        .map(|n| f(n))
        //~^ redundant_closure
        //~| HELP: replace the closure with the function itself
        .map(|n| g(n))
        //~^ redundant_closure
        //~| HELP: replace the closure with the function itself
        .map(|n| S(n))
        //~^ redundant_closure
        //~| HELP: replace the closure with the tuple struct itself
        .map(|n| MyError::A(n))
        //~^ redundant_closure
        //~| HELP: replace the closure with the tuple variant itself
        .unwrap(); // just for nicer formatting
}
