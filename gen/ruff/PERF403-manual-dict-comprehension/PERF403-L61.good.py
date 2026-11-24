def foo():
    result.update({idx: name for idx, name in enumerate(fruit) if idx % 2})  # PERF403
