public class TestType {
  public void middleInlined(
      @TestParameter boolean a, @TestParameter("1") int b, @TestParameter({"x", "y"}) String c) {
    System.out.println(a + " " + b + " " + c);
  }
}
