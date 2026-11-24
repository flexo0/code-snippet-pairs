package p

func fn(x, y interface{}) {
	switch x := x.(type) { //@ diag(`assigning the result of this type assertion`)
	case int:
		fmt.Println(x)
	}
}
