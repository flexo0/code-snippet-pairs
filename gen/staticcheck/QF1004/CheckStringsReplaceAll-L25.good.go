package p

func fn() {
	bytes.Split(nil, nil) //@ diag(`could use bytes.Split instead`)
}
