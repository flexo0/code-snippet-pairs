class Test {
  public int foo(Suit suit) {
    try {
      System.out.println("don't delete 1");
      System.out.println("don't delete 11");
      return switch (suit) {
        case HEART, DIAMOND, SPADE -> 1;
        case CLUB -> throw new NullPointerException();
      };
    } catch (Throwable e) {
      throw new RuntimeException("rethrew");
    }
  }
}
