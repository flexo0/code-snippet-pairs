package p

func fn() {
	b := !(foo() || (bar && !baz)) //@ diag(`merge conditional assignment`)
}
