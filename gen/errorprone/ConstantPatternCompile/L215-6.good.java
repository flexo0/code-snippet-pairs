class Test {
  boolean isCar(String input) {
    return INPUT_PATTERN.matcher(input).matches();
  }
}
