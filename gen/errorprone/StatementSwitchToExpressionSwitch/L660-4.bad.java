class Test {
  public void foo(Suit suit) {
    switch (suit) {
      case HEART:
        System.out.println("heart");
        break;
      case DIAMOND:
        {
          System.out.println("nested1");
          break;
        }
      case SPADE:
      case CLUB:
        System.out.println("everything else");
    }
  }
}
