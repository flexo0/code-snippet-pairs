def issue_19153_1():
    v = {(x,): o for o, (x,) in ["ox"]}
