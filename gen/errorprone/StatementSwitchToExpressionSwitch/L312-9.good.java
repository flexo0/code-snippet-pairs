class Test {
  public void foo(Suit suit) {
    switch (suit) {
      case DIAMOND -> {
        // Diamond break comment
      }
      case SPADE, CLUB -> System.out.println("everything else");
    }
  }
}
