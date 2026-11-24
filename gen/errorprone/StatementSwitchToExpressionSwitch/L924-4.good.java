class Test {
  private void foo(Suit suit) {
    switch (suit) {
      case HEART, DIAMOND ->
          // A comment here
          // more comments.
          // Diamond comment
          System.out.println("Heart or diamond");
      case SPADE, CLUB -> {
        bar();
        System.out.println("Black suit");
      }
    }
  }
}
