class Test {
  public int foo(Suit suit) {
    return switch (suit) {
      case HEART, DIAMOND -> 1;
    };
  }
}
