op_eq = lambda x, y: x == y
op_ne = lambda x, y: x != y
op_lt = lambda x, y: x < y
op_lte = lambda x, y: x <= y
op_gt = lambda x, y: x > y
op_gte = lambda x, y: x >= y
op_is = lambda x, y: x is y
op_isnot = lambda x, y: x is not y
op_in = lambda x, y: y in x
op_itemgetter = lambda x: x[0]
op_itemgetter = lambda x: (x[0], x[1], x[2])
op_itemgetter = lambda x: (x[1:], x[2])
op_itemgetter = lambda x: x[:]
op_itemgetter = lambda x: x[0, 1]
op_itemgetter = lambda x: x[(0, 1)]
