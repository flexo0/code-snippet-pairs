fn main() {
    if u_8 < u8::MAX {
        //~^ implicit_saturating_add
        u_8 += 1;
    }
}
