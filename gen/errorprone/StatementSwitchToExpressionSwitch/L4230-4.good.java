class Test {
  public int foo(Suit suit) {
    x =
        switch (suit) {
          case HEART, DIAMOND ->
              // Heart comment
              (((x + 1) * (x * x)) << 2);
          case SPADE -> throw new RuntimeException();
          case CLUB -> throw new NullPointerException();
        };
  }
}
