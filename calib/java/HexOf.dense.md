# HexOf.asBytes

`byte[] asBytes() throws Exception`: `hex = origin.asString()` decoded, 2 hex digits → 1 byte, the high nibble first.

- `(hex.length() & 1) == 1` → `throw new IOException("Length of hexadecimal text is odd")`.
- `iter` = `hex.chars()` mapped lazily by `c -> Character.digit(c, 16)`; `-1` → `throw new
  IOException("Unexpected character '%c'")`, `c` in it.
- `result = new byte[hex.length() / 2]`; for `index = 0, 2, 4, … < hex.length()`:
  `result[index >>> 1] = (byte) ((iter.next() << 4) + iter.next())`.
- `UncheckedIOException ex` from `iter` → `cause = ex.getCause()`; `cause.addSuppressed(ex)`; `throw cause`.
- `return result`.
