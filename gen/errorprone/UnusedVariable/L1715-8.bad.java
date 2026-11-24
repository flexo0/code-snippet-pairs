public class Test {
  public void f(List<List<String>> lists) {
    List<String> result =
        lists.stream().collect(ArrayList::new, Collection::addAll, Collection::addAll);
    result = lists.stream().collect(ArrayList::new, ArrayList::addAll, ArrayList::addAll);
  }
}
