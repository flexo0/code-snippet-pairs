fn needless_bool_in_the_suggestion_wraps_the_predicate_of_if_else_statement_in_brackets() {
    let x = if b {
        true
    } else if returns_bool() {
        false
    } else {
        true
    };
}
