fn main() {
    let _ = x >= y + 1; //~ int_plus_one
    let _ = x >= 1 + y; //~ int_plus_one
    let _ = y + 1 <= x; //~ int_plus_one
    let _ = 1 + y <= x; //~ int_plus_one
}
