public class TestType {
  public void nullableEnumTest() {
    for (TimeUnit u : asList(TimeUnit.SECONDS, null)) {
      System.out.println(u);
    }
  }
}
