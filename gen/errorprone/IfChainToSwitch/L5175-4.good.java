class Test {
  public int foo(Suit s) {
    switch (s) {
      case Suit.HEART -> {
        return 1;
      }
      case Suit.SPADE -> {
        return 2;
      }
      case Suit.DIAMOND -> {
        return 3;
      }
      case Suit.CLUB -> {
        return 4;
      }
    }
  }
}
