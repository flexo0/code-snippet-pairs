class Test {
  public int foo(Suit suit, boolean b, boolean c) {
    outer:
    do {
      {
        if (b) {
          continue outer;
        }
        return switch (suit) {
          case HEART, DIAMOND -> 1;
          case SPADE -> 2;
          case CLUB -> 3;
        };
      }
    } while (c);
  }
}
