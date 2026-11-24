package p

func fn1() {
	switch x { //@ diag(`could use tagged switch on x`)
	case 1, 2, 3:
	case 4:
	default:
	}
}
