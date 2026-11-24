# destination_length

`destination_length(text: &str) -> usize`: the byte length of the link destination at the start of `text`.

- `text.starts_with('<')` and `Some(end) = text.find('>')` → `end + 1`.
- Else, `depth = 0`, for `(i, c)` in `text.char_indices()`:

| `c` | → |
|-----|---|
| `\` | skip the next `c` |
| `(` | `depth += 1` |
| `)`, `depth == 0` | `return i` |
| `)` | `depth -= 1` |
| `c.is_whitespace()` | `return i` |

- No return → `text.len()`.
