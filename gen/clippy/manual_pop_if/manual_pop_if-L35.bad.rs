fn is_some_and_pattern_positive(mut vec: Vec<i32>, mut deque: VecDeque<i32>, mut heap: BinaryHeap<i32>) {
    if vec.last().is_some_and(|x| *x > 2) {
        vec.pop().expect("element");
    }
}
