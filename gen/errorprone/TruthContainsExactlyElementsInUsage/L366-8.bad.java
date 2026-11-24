public class ExampleClassTest {
  void test() {
    assertThat(ImmutableList.of()).containsExactlyElementsIn(Arrays.asList());
  }
}
