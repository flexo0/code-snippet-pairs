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
        default:
          throw new NullPointerException();
      }
    }
  }
}
