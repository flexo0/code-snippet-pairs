fn main() {
    if s.is_empty()
    //~^ comparison_to_empty
        && s.is_empty()
    //~^ comparison_to_empty
    {}
}
