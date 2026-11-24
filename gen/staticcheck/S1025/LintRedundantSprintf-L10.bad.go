package p

func fn() {
	_ = fmt.Sprintf("%s", t1) //@ diag(`underlying type is a slice of bytes`)
}
