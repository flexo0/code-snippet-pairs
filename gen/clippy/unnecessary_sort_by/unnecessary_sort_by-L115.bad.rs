fn issue16405() {
    v.sort_by(|a, b| a.0.cmp(&b.0));
}
