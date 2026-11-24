class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case HEART:
      case DIAMOND:
        x = ((y + 1) * (y * y)) << 1;
        break;
      case SPADE:
        throw new RuntimeException();
      default:
        throw new NullPointerException();
    }
  }
}
