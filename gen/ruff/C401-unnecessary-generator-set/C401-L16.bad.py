x = set(x for x in range(3))
x = set(
    x for x in range(3)
)
print(f"Hello {set(a for a in range(3))} World")
print(f"{set(a for a in 'abc') - set(a for a in 'ab')}")
print(f"{ set(a for a in 'abc') - set(a for a in 'ab') }")

# Strip parentheses from inner generators.
