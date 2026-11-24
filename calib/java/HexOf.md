# HexOf.asBytes

`HexOf.asBytes()` decodes the hexadecimal text of `origin` into its bytes: two hex digits a byte, the first one the
high half.

It reads the text from `origin`. A text of odd length fails with an `IOException`, "Length of hexadecimal text is
odd".

It maps the text's characters, lazily, to their values as hex digits (`Character.digit`, radix 16). A character
that isn't one fails with an `IOException`, "Unexpected character '<c>'", the character in it.

The result is an array of half the text's length. It fills it a byte at a time, from the start: each byte is the
next digit's value shifted left by four, plus the one after it. A digit's failure comes out of the lazy mapping
wrapped in an `UncheckedIOException`: it unwraps it, adds the wrapper to the cause as suppressed, and throws the
cause.

It returns the array.
