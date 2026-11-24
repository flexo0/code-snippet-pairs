fn swap3() {
    let temp = v[i1];
    //~^ manual_swap
    v[i1] = v[i2];
    v[i2] = temp;
}
