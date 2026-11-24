fn issue16348() {
    v.sort_by(|&&b1, &&b2| b1.cmp(&b2));
}
