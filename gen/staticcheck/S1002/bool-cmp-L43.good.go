package p

func fn() {
	if !y { //@ diag(`simplified to !y`)
	}
}
