class Test {
  private void foo(Suit suit) {
    switch (suit) {
      case HEART:
      // A comment here
      // more comments.
      case DIAMOND:
        // Diamond comment
        System.out.println("Heart or diamond");
        break;
      case SPADE:
      // Fall through
      case CLUB:
        bar();
        System.out.println("Black suit");
    }
  }
}
