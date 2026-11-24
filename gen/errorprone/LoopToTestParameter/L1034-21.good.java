public class TestType {
  public void setOfTest(@TestParameter({"1", "-2", "-3"}) int x) {
    System.out.println(x);
  }
}
