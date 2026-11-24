class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case Suit s when s == Suit.HEART -> throw new NullPointerException();
      default -> throw new NullPointerException();
    }
  }
}
