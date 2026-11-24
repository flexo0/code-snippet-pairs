package p

func example(ch chan int) {
	x := <-ch
	fmt.Println(x)
}
