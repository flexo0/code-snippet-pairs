package p

func fn(t time.Time) {
	time.Until(t) //@ diag(`time.Until`)
}
