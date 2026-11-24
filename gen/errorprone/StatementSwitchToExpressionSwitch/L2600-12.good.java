class Test {
  public int foo(Suit suit, Suit other) {
    return switch (other) {
      case HEART, DIAMOND -> 4;
      case SPADE -> 5;
      case CLUB -> 6;
    };
  }
}
