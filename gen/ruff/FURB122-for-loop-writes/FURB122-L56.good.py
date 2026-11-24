def _():
    with open("file", "w") as f:
        f.writelines(() for () in a)
