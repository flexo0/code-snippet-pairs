class Test {
  public void foo(Suit suit) {
    outer:
    for (; ; ) {
      switch (suit) {
        case HEART:
          System.out.println("will return");
          return;
        case DIAMOND:
          break outer;
        case SPADE:
        case CLUB:
          System.out.println("everything else");
      }
    }
  }
}
