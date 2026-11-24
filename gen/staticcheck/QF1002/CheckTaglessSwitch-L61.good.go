package p

func fn1() {
	switch b := 42; b { //@ diag(`could use tagged switch on b`)
	case 0:
	case 1:
	default:
		_ = b
	}
}
