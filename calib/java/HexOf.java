class HexOf {
    public byte[] asBytes() throws Exception {
        final String hex = this.origin.asString();
        if ((hex.length() & 1) == 1) {
            throw new IOException("Length of hexadecimal text is odd");
        }
        final Iterator<Integer> iter = new Mapped<>(
            c -> {
                final int result = Character.digit(c, 16);
                if (result == -1) {
                    throw new IOException(
                        new FormattedText(
                            "Unexpected character '%c'",
                            c
                        ).asString()
                    );
                }
                return result;
            },
            hex.chars().mapToObj(c -> (char) c).iterator()
        );
        final byte[] result = new byte[hex.length() / 2];
        int index = 0;
        while (index < hex.length()) {
            try {
                result[index >>> 1] = (byte) ((iter.next() << 4) + iter.next());
                index += 2;
            } catch (final UncheckedIOException ex) {
                final IOException cause = ex.getCause();
                cause.addSuppressed(ex);
                throw cause;
            }
        }
        return result;
    }
}
