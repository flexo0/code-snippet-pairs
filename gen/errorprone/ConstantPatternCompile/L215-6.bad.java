class Test {
  boolean isCar(String input) {
    return Pattern.compile("car").matcher(input).matches();
  }
}
