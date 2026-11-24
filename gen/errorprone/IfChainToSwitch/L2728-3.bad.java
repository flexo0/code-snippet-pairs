class Test {
  public void foo(Suit s) {
    {
      Suit suit = s;
      if (Suit.SPADE == suit) {
        return;
      } else if (suit == Suit.DIAMOND) {
        return;
      } else if (suit == Suit.HEART) {
        return;
      } else if (suit == Suit.CLUB) {
        return;
      } else if (suit == null) {
        return;
      }
      System.out.println("this will become unreachable");
      System.out.println("this will too");
    }
    System.out.println("this will vanish");
  }
}
