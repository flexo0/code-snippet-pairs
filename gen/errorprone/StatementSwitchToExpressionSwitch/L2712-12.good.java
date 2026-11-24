class Test {
  public int foo(Suit suit) {
    if (invoke() > 0) {
      return switch (suit) {
        case HEART, DIAMOND, SPADE ->
            /* lhs comment */
            // rhs comment
            // Another comment
            /* sparkly */
            /* Sparkly */
            // Diamond
            // Before invoke
            invoke();
      };
    }
  }
}
