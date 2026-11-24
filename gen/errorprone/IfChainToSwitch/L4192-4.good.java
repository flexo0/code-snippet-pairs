class Test {
  private void foo(Suit suit) {
    switch (suit) {
      case Suit.SPADE -> System.out.println("spade");
      case Suit.HEART, Suit.DIAMOND -> System.out.println("red suit");
      case Suit.CLUB -> System.out.println("club");
    }
  }
}
