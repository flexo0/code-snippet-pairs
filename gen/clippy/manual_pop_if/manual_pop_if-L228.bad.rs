fn map_unwrap_or_pattern_positive(mut vec: Vec<i32>, mut deque: VecDeque<i32>, mut heap: BinaryHeap<i32>) {
    if vec.last().map(|x| *x > 2).unwrap_or(false) {
        vec.pop().expect("element");
    }
}
