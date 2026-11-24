class Test {
  public int foo(Suit suit) {
    final /* chi */ int /* gamma */ x /* delta */; // epsilon
    // zeta
    switch (suit) {
      case HEART:
      case DIAMOND:
        x = ((y + 1) * (y * y)) << 1;
        break;
      case SPADE:
        throw new RuntimeException();
      default:
        throw new NullPointerException();
    }
    Map<? extends String, ? super Test> map = null;
    switch (suit) {
      case HEART:
      case DIAMOND:
        map = new HashMap<>();
        break;
      case SPADE:
        throw new RuntimeException();
      default:
        throw new NullPointerException();
    }
  }
}
