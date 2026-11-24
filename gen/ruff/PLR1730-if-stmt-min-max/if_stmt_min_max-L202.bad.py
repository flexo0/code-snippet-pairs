class Foo:
    def foo(self, value) -> None:
        if value < self._min:
            self._min = value
        if value > self._max:
            self._max = value
