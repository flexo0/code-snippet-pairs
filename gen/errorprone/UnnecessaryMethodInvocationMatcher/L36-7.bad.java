public class Test {
  private static final Matcher<ExpressionTree> TO_STRING =
      methodInvocation(instanceMethod().anyClass().named("toString"));
}
