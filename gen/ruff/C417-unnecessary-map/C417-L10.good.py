(3.0 for _ in nums)
_ = "".join((x in nums and "1" or "0" for x in range(123)))
all((isinstance(v, dict) for v in nums))
filter(func, (v for v in nums))


# When inside f-string, then the fix should be surrounded by whitespace
