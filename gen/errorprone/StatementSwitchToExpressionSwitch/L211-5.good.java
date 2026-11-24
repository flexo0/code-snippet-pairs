class Test {
  public void foo(Suit suit) {
    switch (suit) {
      case HEART -> {
        // Comment before first case
        // Explanatory comment
        System.out.println("this block cannot complete normally");
        {
          throw new NullPointerException();
        }
      }
      case CLUB, SPADE, DIAMOND -> System.out.println("non-heart");
    }
  }
}
