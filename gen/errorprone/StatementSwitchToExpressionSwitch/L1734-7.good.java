class Test {
  public int foo(Suit suit) {
    return switch (suit) {
      case HEART, DIAMOND -> invoke();
      case SPADE -> throw new RuntimeException();
      case CLUB -> throw new NullPointerException();
    };
  }
}
