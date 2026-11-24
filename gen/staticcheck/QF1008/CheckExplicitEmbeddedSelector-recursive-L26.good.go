package p

func fn() {
	_ = t2.F2 //@ diag(`could remove embedded field "T3" from selector`)
}
