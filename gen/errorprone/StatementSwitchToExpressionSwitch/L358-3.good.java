class Test {
  public void foo(Suit suit) {
    for (; ; ) {
      switch (suit) {
        case HEART -> System.out.println("heart");
        case DIAMOND -> {
          continue;
        }
        case SPADE -> {
          return;
        }
        case CLUB -> throw new AssertionError();
      }
    }
  }
}
