class Test {
  public int foo(Suit suit) {
    return switch (suit) {
      case HEART, DIAMOND -> 1;
      case SPADE -> {
        System.out.println("spade");
        throw new RuntimeException();
      }
      case CLUB -> throw new NullPointerException();
      case Suit s when s == Suit.HEART -> throw new NullPointerException();
    };
  }
}
