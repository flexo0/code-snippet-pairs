class Test {
  public boolean equals(Object o) {
    if (!(o instanceof Test other) || other.x != this.x) {
      return false;
    }
  }
}
