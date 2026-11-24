class Test {
  public void foo(Suit suit) {
    switch (suit) {
      case HEART:
        System.out.println("will return");
        return;
      case DIAMOND:
      case SPADE, CLUB:
        System.out.println("everything else");
    }
  }
}
