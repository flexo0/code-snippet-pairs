package p

func fn() {
	if t1 == t2 { //@ diag(`probably want to use time.Time.Equal instead`)
	}
}
