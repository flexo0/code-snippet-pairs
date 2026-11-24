fn destination_length(text: &str) -> usize {
    if text.starts_with('<')
        && let Some(end) = text.find('>')
    {
        return end + 1;
    }
    let mut depth = 0;
    let mut chars = text.char_indices();
    while let Some((i, c)) = chars.next() {
        match c {
            '\\' => {
                chars.next();
            }
            '(' => depth += 1,
            ')' if depth == 0 => return i,
            ')' => depth -= 1,
            c if c.is_whitespace() => return i,
            _ => {}
        }
    }
    text.len()
}
