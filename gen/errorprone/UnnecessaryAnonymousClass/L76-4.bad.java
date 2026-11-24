class Test {
  private static final Function<String, String> F =
      new Function<String, String>() {
        public String apply(String x) {
          return "hello " + x;
        }
      };
}
