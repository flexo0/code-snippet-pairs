class Test {
  public int foo(Suit suit) {
    var suit2 =
        switch (suit) {
          case HEART, DIAMOND -> Suit.SPADE;
          case SPADE -> throw new RuntimeException();
          default -> throw new NullPointerException();
        };
  }
}
