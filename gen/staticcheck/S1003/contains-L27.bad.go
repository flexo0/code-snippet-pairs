package p

func fn() {
	_ = strings.Index("", "") > -1 //@ diag(` strings.Contains`)
	_ = strings.Index("", "") >= 0 //@ diag(` strings.Contains`)
}
