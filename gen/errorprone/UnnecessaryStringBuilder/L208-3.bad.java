abstract class Test {
  void f(String hello) {
    new StringBuilder().append(hello).append(1).toString().getBytes();
  }
}
