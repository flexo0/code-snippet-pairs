class Test {
  public int foo(Suit suit) {
    if (invoke() > 0) {
      switch (suit) {
        case HEART /* lhs comment */: // rhs comment
        // Fall through
        case DIAMOND:
          return invoke();
        case SPADE:
          throw new RuntimeException();
        case CLUB:
          throw new NullPointerException();
      }
      // Custom comment - should never happen
      int z = invoke(/* block comment 0 */ );
      // Custom comment 2
      {
        z++;
      }
      throw new RuntimeException("Switch was not exhaustive at runtime " + z);
    }
  }
}
