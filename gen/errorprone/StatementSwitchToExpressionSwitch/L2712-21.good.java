class Test {
  public int foo(Suit suit) {
    if (invoke() > 0) {
      return switch (suit) {
        case CLUB -> throw new NullPointerException();
      };
    }
  }
}
