class Test {
  public void foo(Suit s) {
    switch (suit.hashCode()) {
      case 213 -> System.out.println("It's a string!");
      case 456 -> System.out.println("It's a diamond!");
      case 789 -> System.out.println("It's a diamond!");
      default -> {}
    }
  }
}
