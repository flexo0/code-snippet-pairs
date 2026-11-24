fn if_let_custom_enum(x: Simple) {
    let _: Simple = if let Simple::A = x {
        //~^ needless_match
        Simple::A
    } else if let Simple::B = x {
        Simple::B
    } else if let Simple::C = x {
        Simple::C
    } else {
        x
    };
}
