# url_length

`url_length(text: &str) -> Option<usize>`: the byte length of the URL at the start of `text`.

1. `scheme` = the length of the prefix matching `[A-Za-z][A-Za-z0-9+.-]*`.
2. `scheme == 0 || !text[scheme..].starts_with("://")` → `None`.
3. `body = scheme + 3`; `end = body + i`, `i` the first char of `text[body..]` in `` \s < > " ` | [ ] ``, else
   `end = text.len()`.
4. While `text[..end]` ends with one of `? ! . , : ; * _ ~ '`, or with `)` and `count(')') > count('(')`:
   `end -= 1`.
5. `end > body` → `Some(end)`, else `None`.
