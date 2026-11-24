public class Test {
  private static final Matcher<ExpressionTree> TO_STRING =
      instanceMethod().anyClass().named("toString");
}
