public class Test {
  String f(int x) {
    return switch (x) {
      case 0 -> "ZERO";
      case 1 -> "ONE";
      case 2 -> {
        // hello
        // world
        System.err.println();
        System.err.println();
        yield "TWO";
      }
    };
  }
}
