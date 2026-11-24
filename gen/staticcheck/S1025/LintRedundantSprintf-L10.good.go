package p

func fn() {
	_ = string(t1) //@ diag(`underlying type is a slice of bytes`)
}
