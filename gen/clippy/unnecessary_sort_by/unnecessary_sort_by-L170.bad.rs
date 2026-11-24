fn issue16348() {
    v.sort_by(|[a1, b1], [a2, b2]| (a1 - b1).cmp(&(a2 - b2)));
}
