var = a_dict.get(key, lambda x=0, /, y=1, *, z=2: (x, y, z))

# OK: dict.get would evaluate the lambda's default even when the key exists.
