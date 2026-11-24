class Test {
  public int foo(Suit suit, Suit other) {
    switch (other) {
      case HEART:
      case DIAMOND:
        return 4;
      case SPADE:
        return 5;
      case CLUB:
        return 6;
    }
    return 0;
  }
}
