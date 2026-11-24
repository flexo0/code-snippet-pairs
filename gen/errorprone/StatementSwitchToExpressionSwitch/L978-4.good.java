class Test {
  private void foo(Suit suit) {
    switch (suit) {
      case HEART, DIAMOND, SPADE, CLUB -> {
        /* red */
        // A comment here
        // more comments.
        /* red */
        // Diamonds comment
        /* black */
        // Spades comment
        /* black */
        bar();
        System.out.println("Any suit");
      }
    }
  }
}
