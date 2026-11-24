fn issue_9583() {
    Some(if 1 > 2 {1} else {2}) == Some(2);
}
