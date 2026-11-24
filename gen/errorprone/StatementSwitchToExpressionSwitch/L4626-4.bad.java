public class Test {
  String f(int x) {
    switch (x) {
      case 0:
        return "ZERO";
      case 1:
        return "ONE";
      case 2: // hello
        // world
        System.err.println();
        System.err.println();
        return "TWO";
    }
  }
}
