abstract class Test {
  private static final String FORMAT = "hello %s";
  private static final String FORMAT2 = "hello %s";

  void h(boolean b) {
    f(FORMAT, 42);
    g(b, FORMAT2, 42);
  }
}
