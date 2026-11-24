def issue_19153_1():
    v = {}
    for o, (x,) in ["ox"]:
        v[x,] = o
