public class TestType {
  public void existingParam(@TestParameter int a, @TestParameter({"1", "2"}) int b) {
    System.out.println(a + b);
  }
}
