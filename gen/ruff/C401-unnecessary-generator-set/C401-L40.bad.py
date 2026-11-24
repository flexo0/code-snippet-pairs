print(t"Hello {set(f(a) for a in 'abc')} World")
print(t"Hello { set(f(a) for a in 'abc') } World")
small_nums = t"{set(a if a < 6 else 0 for a in range(3))}"
print(t"Hello {set(a for a in range(3))} World")
print(t"{set(a for a in 'abc') - set(a for a in 'ab')}")
print(t"{ set(a for a in 'abc') - set(a for a in 'ab') }")


# Not built-in set.
