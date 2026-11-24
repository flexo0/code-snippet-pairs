package p

func fn() {
	_ = s[:len(s)] //@ diag(`omit second index`)
}
