class Test {
  public boolean equals(Object o) {
    if (!(o instanceof Test) || ((Test) o).x != this.x) {
      return false;
    }
    Test other = (Test) o;
  }
}
