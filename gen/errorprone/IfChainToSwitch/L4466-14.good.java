class Test {
  public void foo(Die d) {
    switch (d) {
      case Die.ONE -> System.out.println("One");
      case Die.TWO, Die.FOUR, Die.SIX -> System.out.println("Even");
      case Die.THREE -> System.out.println("Three");
      default -> {}
    }
  }
}
