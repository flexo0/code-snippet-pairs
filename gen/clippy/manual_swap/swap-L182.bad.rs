fn main() {
    let t = b;
    //~^ manual_swap
    b = a;
    a = t;
}
