fn custom_type_match() {
    let _: Simple = match se {
        //~^ needless_match
        Simple::A => Simple::A,
        Simple::B => Simple::B,
        Simple::C => Simple::C,
        Simple::D => Simple::D,
    };
}
