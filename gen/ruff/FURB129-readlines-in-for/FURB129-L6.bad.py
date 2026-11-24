with open("FURB129.py") as f:
    for _line in f.readlines():
        pass
    a = [line.lower() for line in f.readlines()]
    b = {line.upper() for line in f.readlines()}
    c = {line.lower(): line.upper() for line in f.readlines()}
