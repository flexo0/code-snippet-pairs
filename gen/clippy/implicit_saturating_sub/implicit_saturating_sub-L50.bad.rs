fn main() {
    if u_16 > 0 {
        //~^ implicit_saturating_sub
        u_16 -= 1;
    }
}
