class Test {
  private void foo(Suit suit) {
    if (suit == Suit.SPADE) {
      System.out.println("spade");
    } else if (suit == Suit.HEART || suit == Suit.DIAMOND) {
      System.out.println("red suit");
    } else if (suit == Suit.CLUB) {
      System.out.println("club");
    }
  }
}
