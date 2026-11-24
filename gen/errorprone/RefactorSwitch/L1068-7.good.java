class Test {
  public int foo(Suit suit, boolean b) {
    while (true) {
      return switch (suit) {
        case HEART, DIAMOND -> 1;
        case SPADE -> 2;
        case CLUB -> 3;
      };
    }
  }
}
