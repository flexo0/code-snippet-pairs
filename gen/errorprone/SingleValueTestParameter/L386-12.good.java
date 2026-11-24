public class TestType {
  public void middleInlined(@TestParameter boolean a, @TestParameter({"x", "y"}) String c) {
    int b = 1;
    System.out.println(a + " " + b + " " + c);
  }
}
