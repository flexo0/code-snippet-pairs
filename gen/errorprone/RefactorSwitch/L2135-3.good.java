class Test {
  public int foo(Suit suit) {
    return switch (suit) {
      case Suit.HEART -> 1;
      case Suit.CLUB -> 2;
      case Suit.DIAMOND -> 3;
      case Suit.SPADE -> 4;
    };
  }
}
