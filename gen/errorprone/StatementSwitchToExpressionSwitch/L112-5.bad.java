class Test {
  public void foo(Suit suit) {
    switch (suit) {
      case HEART:
        // Explanatory comment
        System.out.println("this block cannot complete normally");
        {
          throw new NullPointerException();
        }
      case CLUB:
      case DIAMOND:
      case SPADE:
        System.out.println("non-heart");
    }
  }
}
