public class TestType {
  public void floatArrayTest(@TestParameter({"1.0", "2.5"}) float f) {
    System.out.println(f);
  }
}
