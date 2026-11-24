public class TestType {
  public void enumArrayTest(@TestParameter({"SECONDS", "MINUTES"}) TimeUnit u) {
    System.out.println(u);
  }
}
