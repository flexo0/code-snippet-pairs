package p

func fn() {
	b := true //@ diag(`merge conditional assignment`)
	if foo() || (bar && !baz) {
		b = false
	}
}
