class Test {
  public int foo(Suit suit) {
    return switch (suit) {
      case HEART -> 1;
      case DIAMOND -> 2;
      case SPADE -> 3;
      case CLUB -> 4;
    };
  }
}
