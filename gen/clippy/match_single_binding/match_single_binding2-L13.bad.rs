fn main() {
    fn size_hint<I: Iterator>(iter: &AppendIter<I>) -> (usize, Option<usize>) {
        match &iter.inner {
            Some((iter, _item)) => match iter.size_hint() {
                //~^ match_single_binding
                (min, max) => (min.saturating_add(1), max.and_then(|max| max.checked_add(1))),
            },
            None => (0, Some(0)),
        }
    }
}
