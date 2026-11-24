fn xor_swap_locals() {
    a ^= b;
    //~^ manual_swap
    b ^= a;
    a ^= b;
}
