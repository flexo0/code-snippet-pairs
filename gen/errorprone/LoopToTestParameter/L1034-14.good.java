public class TestType {
  public void listOfTest(@TestParameter({"true", "false"}) String s) {
    System.out.println(s);
  }
}
