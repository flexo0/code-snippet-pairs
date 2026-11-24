fn issue_14931() {
    for i2 in 0..4 {
        v.swap(i1, i2);

        i1 += 2;
    }
}
