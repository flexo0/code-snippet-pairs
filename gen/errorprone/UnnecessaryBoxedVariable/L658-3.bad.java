class Test {
  private void a() {
    Integer y = Integer.valueOf("42");
    y = Integer.valueOf("43");
    y = Integer.valueOf(43);
    y = new Integer("44");
    y = new Integer(44);
  }
}
