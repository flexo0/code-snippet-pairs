fn xor_slice_swap() {
    foo[0] ^= foo[1];
    //~^ manual_swap
    foo[1] ^= foo[0];
    foo[0] ^= foo[1];
}
