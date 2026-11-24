# destination_length

`destination_length(text)` gives the length, in bytes, of the link destination that `text` starts with: the text
right after a link's `](`.

If `text` starts with `<` and has a `>`, the destination is everything up to and including the first `>`.

Otherwise it walks the text a character at a time, keeping a depth of open parentheses, from 0:

- a `\` skips the character after it;
- a `(` opens one more;
- a `)` at depth 0 ends the destination, before it; a deeper one closes one;
- a whitespace character ends the destination, before it;
- anything else is part of it.

With no end found, the destination is the whole text.
