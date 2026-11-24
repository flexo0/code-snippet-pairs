class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case Suit.HEART:
        return 1;
      case Suit.CLUB:
        return 2;
      case Suit.DIAMOND:
        return 3;
      case Suit.SPADE:
        return 4;
    }
    throw new AssertionError();
  }
}
