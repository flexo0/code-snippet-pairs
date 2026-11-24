class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case HEART:
      case DIAMOND:
        return invoke();
      case SPADE:
        throw new RuntimeException();
      default:
        throw new NullPointerException();
    }
  }
}
