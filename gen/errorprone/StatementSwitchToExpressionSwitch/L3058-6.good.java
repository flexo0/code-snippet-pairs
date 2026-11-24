class Test {
  public int foo(Suit suit) {
    x =
        switch (suit) {
          case HEART, DIAMOND -> ((y + 1) * (y * y)) << 1;
          case SPADE -> throw new RuntimeException();
          default -> throw new NullPointerException();
        };
  }
}
