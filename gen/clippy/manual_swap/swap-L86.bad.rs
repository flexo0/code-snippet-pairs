fn xor_field_swap() {
    bar.a ^= bar.b;
    //~^ manual_swap
    bar.b ^= bar.a;
    bar.a ^= bar.b;
}
