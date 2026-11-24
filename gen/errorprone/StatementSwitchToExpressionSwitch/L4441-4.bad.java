class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case /* red suit */ HEART:
      // Heart comment
      case /* red suit */ DIAMOND: // sparkles
      // Diamond comment
      // Fall through
      case /* black suit */ SPADE:
        x *= 2;
        // Before break comment
        break;
      // After break comment
      case /* black suit */ CLUB:
        // Club comment
        throw new NullPointerException();
        // Club after throw comment
    }
  }
}
