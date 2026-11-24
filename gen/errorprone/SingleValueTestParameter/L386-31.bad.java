public class TestType {
  public void leadingInlinedBeforeVarargs(@TestParameter("foo") String first, String... rest) {
    System.out.println(first + rest.length);
  }
}
