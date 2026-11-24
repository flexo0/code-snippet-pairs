public class ExampleClassTest {
  void test() {
    assertThat(ImmutableList.of(1, 2, 3)).containsExactlyElementsIn(new Integer[] {1, 2, 3});
  }
}
