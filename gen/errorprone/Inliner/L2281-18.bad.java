public final class Client {
  void test() {
    try {
      // TODO(b/583897542): Do not drop an argument expression that throws a checked
      // exception caught by an enclosing try-catch block (or side-effecting call).
      call(readValue());
    } catch (IOException e) {
      throw new RuntimeException(e);
    }
  }
}
