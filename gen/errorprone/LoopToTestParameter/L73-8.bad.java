public class TestType {
  public void myTest() {
    for (TimeUnit e : EnumSet.allOf(TimeUnit.class)) {
      System.out.println(e);
    }
  }
}
