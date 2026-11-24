public class TestType {
  public void enumArrayTest() {
    for (TimeUnit u : new TimeUnit[] {TimeUnit.SECONDS, TimeUnit.MINUTES}) {
      System.out.println(u);
    }
  }
}
