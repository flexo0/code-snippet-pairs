fn main() {
    let _ = x as usize - 1 >= z; //~ int_plus_one
    let _ = z <= x as usize - 1; //~ int_plus_one
}
