class Test {
  public void foo(Suit suit) {
    switch (suit) {
      case HEART -> System.out.println("heart");
      case DIAMOND -> System.out.println("nested1");
      case SPADE, CLUB -> System.out.println("everything else");
    }
  }
}
