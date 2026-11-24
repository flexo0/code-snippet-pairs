fn url_length(text: &str) -> Option<usize> {
    let scheme = text
        .char_indices()
        .find(|&(i, c)| {
            !(c.is_ascii_alphabetic()
                || (i > 0 && (c.is_ascii_digit() || matches!(c, '+' | '-' | '.'))))
        })
        .map_or(text.len(), |(i, _)| i);
    if scheme == 0 || !text[scheme..].starts_with("://") {
        return None;
    }
    let body = scheme + 3;
    let mut end = text[body..]
        .find(|c: char| c.is_whitespace() || matches!(c, '<' | '>' | '"' | '`' | '|' | '[' | ']'))
        .map_or(text.len(), |i| body + i);
    loop {
        let url = &text[..end];
        let punctuation = url.ends_with(['?', '!', '.', ',', ':', ';', '*', '_', '~', '\'']);
        let unmatched = url.ends_with(')') && url.matches(')').count() > url.matches('(').count();
        if !(punctuation || unmatched) {
            break;
        }
        end -= 1;
    }
    (end > body).then_some(end)
}
