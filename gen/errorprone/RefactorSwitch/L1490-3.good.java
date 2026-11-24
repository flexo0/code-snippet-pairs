class Test {
  public int foo(Suit suit, boolean c) {
    outer:
    while (true) {
      {
        return switch (suit) {
          case HEART, DIAMOND -> 1;
          case SPADE -> 2;
          case CLUB -> 3;
        };
      }
    }
  }
}
