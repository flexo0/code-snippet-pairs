def foo():
    with open("foo.txt", "r") as f:
        x = f.read()
    return x  # RET504
