class Test {
  public void foo(Suit suit) {
    switch (suit) {
      case DIAMOND:
        // Diamond break comment
        break;
      case SPADE:
      case CLUB:
        System.out.println("everything else");
    }
  }
}
