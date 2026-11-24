class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case HEART:
        return 1;
      case DIAMOND:
        return 2;
      case SPADE:
        return 3;
      case CLUB:
        return 4;
    }
    // LINT.
    throw new AssertionError("unreachable!");
  }
}
