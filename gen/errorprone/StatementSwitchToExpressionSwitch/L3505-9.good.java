class Test {
  public int foo(Suit suit) {
    this.x <<=
        switch (suit) {
          case HEART ->
              /* Comment before first case */
              /* LHS comment */
              // Inline comment
              2;
          case DIAMOND -> (((x + 1) * (x * x)) << 1);
          case SPADE -> throw new RuntimeException();
          default -> throw new NullPointerException();
        };
  }
}
