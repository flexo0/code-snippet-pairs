class Test {
  boolean isMemberSelect(Tree tree) {
    return tree.getKind().equals(Tree.Kind.MEMBER_SELECT);
  }
}
