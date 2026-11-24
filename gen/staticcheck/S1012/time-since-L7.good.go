package p

func fn() {
	_ = time.Since(t1) //@ diag(`time.Since`)
}
