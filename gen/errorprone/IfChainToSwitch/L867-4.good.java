class Test {
  public void foo(Suit suit) {
    switch (suit) {
      case Suit.CLUB -> System.out.println("It's a club!");
      case Suit.DIAMOND -> System.out.println("It's a diamond!");
      case Suit.HEART -> System.out.println("It's a heart!");
      case null, default ->
          // c1
          System.out.println("It's a diamond!");
    }
  }
}
