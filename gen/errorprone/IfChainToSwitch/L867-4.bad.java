class Test {
  public void foo(Suit suit) {
    if (suit == Suit.CLUB) {
      System.out.println("It's a club!");
    } else if (suit == Suit.DIAMOND) {
      System.out.println("It's a diamond!");
    } else if ((suit == Suit.HEART)) {
      System.out.println("It's a heart!");
    } else { // c1
      {
        System.out.println("It's a diamond!");
      }
    }
  }
}
