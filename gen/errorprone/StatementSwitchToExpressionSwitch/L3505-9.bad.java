class Test {
  public int foo(Suit suit) {
    switch (suit) {
      /* Comment before first case */
      case /* LHS comment */ HEART:
        // Inline comment
        this.x <<= 2;
        break;
      case DIAMOND:
        x <<= (((x + 1) * (x * x)) << 1);
        break;
      case SPADE:
        throw new RuntimeException();
      default:
        throw new NullPointerException();
    }
  }
}
