class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case HEART, DIAMOND -> {
        x = x + 1;
      }
      case SPADE -> throw new RuntimeException();
      case CLUB -> throw new NullPointerException();
      default -> throw new IllegalArgumentException();
    }
  }
}
