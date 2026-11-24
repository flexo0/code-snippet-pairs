package p

func fn1() {
	switch b := 42; { //@ diag(`could use tagged switch on b`)
	case b == 0:
	case b == 1:
	default:
		_ = b
	}
}
