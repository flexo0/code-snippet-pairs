"a,b,c".split(',', maxsplit=0) # ["a,b,c"]
"a,b,c".split(',', maxsplit=-1)  # ["a", "b", "c"]
"a,b,c".split(',', maxsplit=-2)  # ["a", "b", "c"]
"a,b,c".split(',', maxsplit=-0)  # ["a,b,c"]

# negatives

# invalid values should not cause panic
