class Test {
  public int foo(Suit suit) {
    if (invoke() > 0) {
      return switch (suit) {
        case HEART, DIAMOND ->
            /* lhs comment */
            // rhs comment
            invoke();
        case SPADE -> throw new RuntimeException();
        case CLUB -> throw new NullPointerException();
      };
    }
  }
}
