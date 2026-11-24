public class TestType {
  public void leadingInlinedBeforeVarargs(String... rest) {
    String first = "foo";
    System.out.println(first + rest.length);
  }
}
