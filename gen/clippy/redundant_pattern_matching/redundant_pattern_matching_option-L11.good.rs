fn issue_11174_edge_cases<T>(boolean: bool, boolean2: bool, maybe_some: Option<T>) {
    let _ = maybe_some.is_none() && (boolean || boolean2); // guard needs parentheses
}
