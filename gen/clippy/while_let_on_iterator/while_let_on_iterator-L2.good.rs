#![expect(
    clippy::redundant_closure_call,
    clippy::single_range_in_vec_init,
    clippy::useless_vec
)]

fn base() {
    let mut iter = 1..20;
    for x in iter {
        //~^ while_let_on_iterator
        println!("{}", x);
    }

    let mut iter = 1..20;
    for x in iter {
        //~^ while_let_on_iterator
        println!("{}", x);
    }

    let mut iter = 1..20;
    for _ in iter {}
    //~^ while_let_on_iterator

    let mut iter = 1..20;
    while let None = iter.next() {} // this is fine (if nonsensical)

    let mut iter = 1..20;
    if let Some(x) = iter.next() {
        // also fine
        println!("{}", x)
    }

    // the following shouldn't warn because it can't be written with a for loop
    let mut iter = 1u32..20;
    while let Some(_) = iter.next() {
        println!("next: {:?}", iter.next())
    }

    // neither can this
    let mut iter = 1u32..20;
    while let Some(_) = iter.next() {
        println!("next: {:?}", iter.next());
    }

    // or this
    let mut iter = 1u32..20;
    while let Some(_) = iter.next() {
        iter = 1..20;
    }
}
