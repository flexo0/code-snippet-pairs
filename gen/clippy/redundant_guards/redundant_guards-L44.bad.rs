fn main() {
    match c {
        C(x, y) if let 1 = y => ..,
        //~^ redundant_guards
        _ => todo!(),
    };
}
