package p

func fn(t time.Time) {
	t.Sub(time.Now()) //@ diag(`time.Until`)
}
