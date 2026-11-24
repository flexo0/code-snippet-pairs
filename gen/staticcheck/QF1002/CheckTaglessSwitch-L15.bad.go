package p

func fn1() {
	switch { //@ diag(`could use tagged switch on x`)
	case x == 1 || x == 2, x == 3:
	case x == 4:
	default:
	}
}
