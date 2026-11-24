def remove_suffix_multiple_attribute_expr() -> None:
    x = foo.bar.baz[:-len(SUFFIX)] if foo.bar.baz.endswith(SUFFIX) else foo.bar.baz
