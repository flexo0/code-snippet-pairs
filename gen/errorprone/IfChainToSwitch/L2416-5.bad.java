class Test {
  public void foo(Suit s) {
    if (suit == Suit.SPADE) {
      return;
    } else if (suit == Suit.DIAMOND) {
      return;
    } else if (suit == Suit.HEART) {
      return;
    } else if (suit == null) {
      throw new NullPointerException();
    }
    return;
  }
}
