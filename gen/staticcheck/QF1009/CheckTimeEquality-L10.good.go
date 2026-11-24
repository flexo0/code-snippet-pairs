package p

func fn() {
	if t1.Equal(t2) { //@ diag(`probably want to use time.Time.Equal instead`)
	}
}
