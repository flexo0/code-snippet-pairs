class Test {
  public int foo(Suit suit) {
    if (invoke() > 0) {
      switch (suit) {
        case HEART /* lhs comment */: // rhs comment
        // Another comment
        case /* sparkly */ DIAMOND /* Sparkly */:
        // Diamond
        case SPADE:
          // Before invoke
          return invoke();
      }
    }
  }
}
