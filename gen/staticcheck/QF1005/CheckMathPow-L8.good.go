package p

func fn() {
	_ = 1.0       //@ diag(`could expand call to math.Pow`)
	_ = x         //@ diag(`could expand call to math.Pow`)
	_ = x * x     //@ diag(`could expand call to math.Pow`)
	_ = x * x * x //@ diag(`could expand call to math.Pow`)
}
