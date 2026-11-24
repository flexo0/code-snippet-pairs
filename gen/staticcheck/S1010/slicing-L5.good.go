package p

func fn() {
	_ = s[:] //@ diag(`omit second index`)
}
