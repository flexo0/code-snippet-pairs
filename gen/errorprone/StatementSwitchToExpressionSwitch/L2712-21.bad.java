class Test {
  public int foo(Suit suit) {
    if (invoke() > 0) {
      switch (suit) {
        case CLUB:
          throw new NullPointerException();
      }
    }
  }
}
