public class TestType {
  public void multipleInlinedWithOneKept(@TestParameter boolean kept) {
    int a = 1;
    String b = "foo";
    System.out.println(a + " " + kept + " " + b);
  }
}
