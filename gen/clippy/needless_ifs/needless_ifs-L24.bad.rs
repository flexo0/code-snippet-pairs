fn main() {
    if (true) {}
    //~^ needless_ifs
    // Do not remove the condition
    if maybe_side_effect() {}
}
