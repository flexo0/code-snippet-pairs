public class ExampleClassTest {
  void test() {
    assertThat(ImmutableList.of(1, 2, 3)).containsExactly(1, 2, 3);
  }
}
