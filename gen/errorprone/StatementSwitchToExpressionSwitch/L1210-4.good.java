public class Test {
  public static void main(String[] args) {
    switch (args[0]) {
      case String s when s.startsWith("a sale") -> System.out.println("it all starts with a sale");
      case "one" -> System.out.println("one");
      case "two", "three" -> System.out.println("two or three");
      case String s -> System.out.println("some other string");
    }
  }
}
