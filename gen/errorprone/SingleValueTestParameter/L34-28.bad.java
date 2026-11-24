public class TestType {
  public void emptyString(@TestParameter("") String s) {
    System.out.println(s);
  }
}
