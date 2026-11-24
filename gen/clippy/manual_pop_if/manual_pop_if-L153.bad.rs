fn let_chain_pattern_positive(mut vec: Vec<i32>, mut deque: VecDeque<i32>, mut heap: BinaryHeap<i32>) {
    if let Some(x) = vec.last() //~ manual_pop_if
        && *x > 2
    {
        vec.pop().unwrap();
    }
}
