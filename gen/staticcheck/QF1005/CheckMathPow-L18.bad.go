package p

func fn() {
	_ = math.Pow(a, 2)     //@ diag(`could expand call to math.Pow`)
	_ = math.Pow(b, 2)     //@ diag(`could expand call to math.Pow`)
	_ = math.Pow(c, 2)     //@ diag(`could expand call to math.Pow`)
	_ = math.Pow(a*1.0, 2) //@ diag(`could expand call to math.Pow`)
}
