class Test {
  public Object test() {
    doAnswer(
            a -> {
              when(test.test()).thenReturn(null);
              return null;
            })
        .when(test)
        .test();
    return null;
  }
}
