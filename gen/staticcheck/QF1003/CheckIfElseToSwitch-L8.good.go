package p

func fn() {
	switch x {
	case 1, 2: //@ diag(`could use tagged switch on x`)
	case 3:
	}
}
