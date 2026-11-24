public class Test {
  public static void main(String[] args) {
    switch (args[0]) {
      case String s
      when s.startsWith("a sale"):
        {
          System.out.println("it all starts with a sale");
          break;
        }
      case "one":
        System.out.println("one");
        break;
      case "two", "three":
        System.out.println("two or three");
        break;
      case String s:
        System.out.println("some other string");
        break;
    }
  }
}
