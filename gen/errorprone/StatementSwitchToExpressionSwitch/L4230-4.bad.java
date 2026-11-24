class Test {
  public int foo(Suit suit) {
    switch (suit) {
      case HEART:
      // Heart comment
      // Fall through
      case DIAMOND:
        x = (((x + 1) * (x * x)) << 2);
        break;
      case SPADE:
        throw new RuntimeException();
      default:
        // This is unlikely to be reached
        throw new RuntimeException();
      case CLUB:
        throw new NullPointerException();
    }
  }
}
