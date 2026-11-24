public class TestType {
  public void myTest(@TestParameter TimeUnit e1) {
    for (TimeUnit e2 : TimeUnit.values()) {
      System.out.println(e1 + " " + e2);
    }
  }
}
