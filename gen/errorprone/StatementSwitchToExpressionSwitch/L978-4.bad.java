class Test {
  private void foo(Suit suit) {
    switch (suit) {
      case /* red */ HEART:
      // A comment here
      // more comments.
      case /* red */ DIAMOND:
      // Diamonds comment
      case /* black */ SPADE:
      // Spades comment
      case /* black */ CLUB:
        bar();
        System.out.println("Any suit");
    }
  }
}
