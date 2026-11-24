class Test {
  public int foo(Suit suit) {
    final int x =
        switch (suit) {
          case HEART, DIAMOND -> ((y + 1) * (y * y)) << 1;
          case SPADE -> throw new RuntimeException();
          default -> throw new NullPointerException();
        };

    Map<? extends String, ? super Test> map =
        switch (suit) {
          case HEART, DIAMOND -> new HashMap<>();
          case SPADE -> throw new RuntimeException();
          default -> throw new NullPointerException();
        };
  }
}
