fn issue_17804() {
    Some(Inner(0)).filter(|x| x.peek());
}
