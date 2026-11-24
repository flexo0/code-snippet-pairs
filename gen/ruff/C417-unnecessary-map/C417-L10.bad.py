map(lambda _: 3.0, nums)
_ = "".join(map(lambda x: x in nums and "1" or "0", range(123)))
all(map(lambda v: isinstance(v, dict), nums))
filter(func, map(lambda v: v, nums))


# When inside f-string, then the fix should be surrounded by whitespace
