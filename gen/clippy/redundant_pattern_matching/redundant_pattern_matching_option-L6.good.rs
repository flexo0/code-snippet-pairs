fn issue_11174<T>(boolean: bool, maybe_some: Option<T>) -> bool {
    maybe_some.is_none() && (!boolean)
    //~^ redundant_pattern_matching
}
