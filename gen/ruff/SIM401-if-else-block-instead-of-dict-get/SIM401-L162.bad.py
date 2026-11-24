if key in a_dict:
    var = a_dict[key]
else:
    var = lambda x=0, /, y=1, *, z=2: (x, y, z)

# OK: dict.get would evaluate the lambda's default even when the key exists.
