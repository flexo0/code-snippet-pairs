fn issue_14931() {
    for i2 in 0..4 {
        let tmp = v[i1];
        //~^ manual_swap
        v[i1] = v[i2];
        v[i2] = tmp;

        i1 += 2;
    }
}
