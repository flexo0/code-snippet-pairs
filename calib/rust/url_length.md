# url_length

`url_length(text)` gives the length, in bytes, of the URL that `text` starts with, or none if it doesn't start
with one.

The scheme comes first: ASCII letters, and after the first character also digits, `+`, `-` and `.`, up to the
first other character or the end. With no scheme (the text starts with another character), or no `://` right after
it, there is no URL.

The URL's body starts after the `://` and runs up to the first whitespace or one of `<`, `>`, `"`, `` ` ``, `|`,
`[`, `]`, or to the end of the text.

Then it trims the URL's end a character at a time, as long as the URL ends with:

- a sentence's punctuation, one of `?`, `!`, `.`, `,`, `:`, `;`, `*`, `_`, `~` and `'`;
- or a `)` that is unmatched: the URL holds more `)` than `(`.

The URL is the text up to the end that is left, if its body still has a character in it; else there is none.
