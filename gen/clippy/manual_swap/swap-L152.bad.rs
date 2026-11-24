fn main() {
    ; let t = c.0;
    //~^ manual_swap
    c.0 = a;
    a = t;
}
