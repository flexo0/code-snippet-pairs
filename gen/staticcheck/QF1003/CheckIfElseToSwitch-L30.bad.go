package p

func fn() {
	for {
		if x == 1 || x == 2 { //@ diag(`could use tagged switch on x`)
		} else if x == 3 {
		}
	}
}
