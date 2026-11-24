public class TestType {
  public void nullableEnumTest(@TestParameter({"SECONDS", "null"}) TimeUnit u) {
    System.out.println(u);
  }
}
