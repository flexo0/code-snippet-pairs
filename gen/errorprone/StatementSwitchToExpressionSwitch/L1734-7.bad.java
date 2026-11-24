class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case HEART:
      case DIAMOND:
        return invoke();
      case SPADE:
        throw new RuntimeException();
      case CLUB:
        throw new NullPointerException();
    }
    // This should never happen
    int z = invoke();
    z++;
    throw new RuntimeException("Switch was not exhaustive at runtime " + z);
  }
}
