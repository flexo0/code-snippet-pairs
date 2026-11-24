package p

func fn() {
	bytes.SplitN(nil, nil, -1) //@ diag(`could use bytes.Split instead`)
}
