fn issue_11174_edge_cases<T>(boolean: bool, boolean2: bool, maybe_some: Option<T>) {
    let _ = matches!(maybe_some, None if boolean || boolean2); // guard needs parentheses
}
