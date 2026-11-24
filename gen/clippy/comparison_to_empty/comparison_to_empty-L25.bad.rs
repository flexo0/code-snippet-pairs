fn main() {
    if let [] = &*s
    //~^ comparison_to_empty
        && s == []
    //~^ comparison_to_empty
    {}
}
