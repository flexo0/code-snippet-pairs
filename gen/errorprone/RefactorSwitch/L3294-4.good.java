class Test {
  public int foo(Suit suit) {
    x =
        switch (suit) {
          case HEART, DIAMOND -> x + 1;
          case SPADE -> throw new RuntimeException();
          case CLUB -> throw new NullPointerException();
        };
  }
}
