with open("FURB129.py") as f:
    for _line in f:
        pass
    a = [line.lower() for line in f]
    b = {line.upper() for line in f}
    c = {line.lower(): line.upper() for line in f}
