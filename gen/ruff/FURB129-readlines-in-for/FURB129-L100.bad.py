with open("furb129.py") as f:
    for line in (((f))).readlines():
        pass
    for line in(f).readlines():
        pass

    # Test case for issue #17683 (missing space before keyword)
    print([line for line in f.readlines()if True])

# https://github.com/astral-sh/ruff/issues/18843
