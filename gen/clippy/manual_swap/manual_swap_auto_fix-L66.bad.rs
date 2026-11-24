fn swap7() {
    let tmp = v[i1 * 3];
    //~^ manual_swap
    v[i1 * 3] = v[i2 / 2];
    v[i2 / 2] = tmp;
}
