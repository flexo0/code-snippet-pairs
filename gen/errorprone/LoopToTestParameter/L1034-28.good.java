public class TestType {
  public void immutableSetOfTest(@TestParameter({"1.5", "-2.5"}) double d) {
    System.out.println(d);
  }
}
