fn issue_11174<T>(boolean: bool, maybe_some: Option<T>) -> bool {
    matches!(maybe_some, None if !boolean)
    //~^ redundant_pattern_matching
}
