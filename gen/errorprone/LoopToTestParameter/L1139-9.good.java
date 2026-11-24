public class TestType {
  public void longArrayTest(@TestParameter({"10", "-20"}) long l) {
    System.out.println(l);
  }
}
