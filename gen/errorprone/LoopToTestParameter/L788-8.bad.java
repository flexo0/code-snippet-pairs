public class TestType {
  public void myTest() {
    for (TimeUnit e1 : TimeUnit.values()) {
      for (TimeUnit e2 : TimeUnit.values()) {
        System.out.println(e1 + " " + e2);
      }
    }
  }
}
