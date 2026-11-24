class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case HEART, DIAMOND -> {
        return 1;
      }
    }
  }
}
