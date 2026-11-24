fn if_let_pattern_positive(mut vec: Vec<i32>, mut deque: VecDeque<i32>, mut heap: BinaryHeap<i32>) {
    if let Some(x) = vec.last() {
        if *x > 2 {
            vec.pop().expect("element");
        }
    }
}
