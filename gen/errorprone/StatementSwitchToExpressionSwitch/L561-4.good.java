class Test {
  public void foo(Suit suit) {
    switch (suit) {
      case HEART -> {
        System.out.println("will return");
        return;
      }
      case DIAMOND, SPADE, CLUB -> System.out.println("everything else");
    }
  }
}
