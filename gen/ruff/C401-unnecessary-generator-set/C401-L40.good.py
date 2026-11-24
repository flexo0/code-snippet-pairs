print(t"Hello { {f(a) for a in 'abc'} } World")
print(t"Hello { {f(a) for a in 'abc'} } World")
small_nums = t"{ {a if a < 6 else 0 for a in range(3)} }"
print(t"Hello {set(range(3))} World")
print(t"{set('abc') - set('ab')}")
print(t"{ set('abc') - set('ab') }")


# Not built-in set.
