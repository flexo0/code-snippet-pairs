fn is_some_and_pattern_positive(mut vec: Vec<i32>, mut deque: VecDeque<i32>, mut heap: BinaryHeap<i32>) {
    vec.pop_if(|x| *x > 2);
}
