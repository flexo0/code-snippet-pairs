class Test {
  public int foo(Suit suit) {
    var suit2 = Suit.SPADE;
    switch (suit) {
      case HEART, DIAMOND -> {
        suit2 = Suit.SPADE;
      }
      case SPADE -> throw new RuntimeException();
      default -> throw new NullPointerException();
    }
  }
}
