package p

func fn() {
	x := false //@ diag(`merge conditional assignment`)
	if foo() || (bar && !baz) {
		x = true
	}
}
