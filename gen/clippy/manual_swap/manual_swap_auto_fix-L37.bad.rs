fn swap4() {
    let temp = v[i1];
    //~^ manual_swap
    v[i1] = v[i2 + 1];
    v[i2 + 1] = temp;
}
