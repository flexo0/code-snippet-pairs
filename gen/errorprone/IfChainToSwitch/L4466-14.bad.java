class Test {
  public void foo(Die d) {
    if (d == Die.ONE) {
      System.out.println("One");
    } else if (d == Die.TWO || ((d == Die.FOUR) || d == Die.SIX)) {
      System.out.println("Even");
    } else if (d == Die.THREE) {
      System.out.println("Three");
    }
  }
}
