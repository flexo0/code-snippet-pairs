package p

func fn() {
	_ = time.Now().Sub(t1) //@ diag(`time.Since`)
}
