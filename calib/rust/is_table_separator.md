# is_table_separator

`is_table_separator(line)` tells whether a line of Markdown is a table's separator line, the one under its header
row, like `|---|:--:|`.

It trims the line's surrounding whitespace first. A line without a `|` is no separator. Otherwise it drops one
leading `|` and one trailing `|`, if there are, and splits what is left on `|` into cells. The line is a separator
when every cell, trimmed of whitespace and then of one leading and one trailing `:`, is not empty and is hyphens
only.
