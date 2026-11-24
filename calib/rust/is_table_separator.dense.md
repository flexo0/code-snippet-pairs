# is_table_separator

`is_table_separator(line: &str) -> bool`, on `line = line.trim()`:

- `!line.contains('|')` → `false`;
- `inner` = `line` less one leading `|`, less one trailing `|`;
- `true` ⇔ ∀ `cell` ∈ `inner.split('|')`: `cell.trim()` less one leading `:`, less one trailing `:`, is `!= ""` and
  all `b'-'`.
