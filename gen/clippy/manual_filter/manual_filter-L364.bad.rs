fn issue_17804() {
    Some(Inner(0)).and_then(|x| if x.peek() { Some(x) } else { None });
}
