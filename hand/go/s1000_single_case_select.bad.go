package p

func example(ch chan int) {
	select {
	case x := <-ch:
		fmt.Println(x)
	}
}
