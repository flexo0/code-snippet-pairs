print("S\x1cP\x1dL\x1eI\x1fT".split())
print("\x1c\x1d\x1e\x1f>".split(maxsplit=0))
print("<\x1c\x1d\x1e\x1f".rsplit(maxsplit=0))

# leading/trailing whitespace should not count towards maxsplit
