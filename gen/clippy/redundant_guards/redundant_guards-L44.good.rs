fn main() {
    match c {
        C(x, 1) => ..,
        //~^ redundant_guards
        _ => todo!(),
    };
}
