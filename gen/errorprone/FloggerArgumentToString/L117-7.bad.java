class Test {
  public void f() {
    logger.atInfo().log("hello '%s'", toString());
  }
}
