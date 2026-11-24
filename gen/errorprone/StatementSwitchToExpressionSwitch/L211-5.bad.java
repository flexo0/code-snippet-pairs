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
      case SPADE:
      case DIAMOND:
        System.out.println("non-heart");
        break;
      default:
        System.out.println("default");
    }
  }
}
