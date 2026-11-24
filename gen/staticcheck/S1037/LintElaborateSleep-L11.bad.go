package p

func fn() {
	select { //@ diag(`should use time.Sleep`)
	case <-time.After(0):
	}
}
