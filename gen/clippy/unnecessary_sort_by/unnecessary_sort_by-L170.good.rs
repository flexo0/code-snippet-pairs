fn issue16348() {
    v.sort_by_key(|[a1, b1]| a1 - b1);
}
