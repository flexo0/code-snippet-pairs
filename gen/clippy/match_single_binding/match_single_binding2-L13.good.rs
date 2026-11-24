fn main() {
    fn size_hint<I: Iterator>(iter: &AppendIter<I>) -> (usize, Option<usize>) {
        match &iter.inner {
            Some((iter, _item)) => {
                let (min, max) = iter.size_hint();
                (min.saturating_add(1), max.and_then(|max| max.checked_add(1)))
            },
            None => (0, Some(0)),
        }
    }
}
