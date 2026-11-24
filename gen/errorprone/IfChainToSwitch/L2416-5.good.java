class Test {
  public void foo(Suit s) {
    switch (suit) {
      case Suit.SPADE -> {
        return;
      }
      case Suit.DIAMOND -> {
        return;
      }
      case Suit.HEART -> {
        return;
      }
      case null -> throw new NullPointerException();
      default -> {
        return;
      }
    }
  }
}
