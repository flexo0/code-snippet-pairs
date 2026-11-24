package p

func fn() {
	x := foo() || (bar && !baz) //@ diag(`merge conditional assignment`)
}
