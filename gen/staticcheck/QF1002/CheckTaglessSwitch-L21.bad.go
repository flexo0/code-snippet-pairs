package p

func fn1() {
	switch { //@ diag(`could use tagged switch on z[""][0]`)
	case z[""][0] == 1 || z[""][0] == 2:
	}
}
