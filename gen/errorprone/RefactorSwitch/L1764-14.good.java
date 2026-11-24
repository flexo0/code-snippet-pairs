class Test {
  public int foo(Suit suit) {
    return switch (suit) {
      case Suit s when s == Suit.HEART -> throw new NullPointerException();
    };
  }
}
