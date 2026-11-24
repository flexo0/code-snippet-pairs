public class TestType {
  public void myTest(@TestParameter({"a", "b", "c"}) String s) {
    System.out.println(s);
  }
}
