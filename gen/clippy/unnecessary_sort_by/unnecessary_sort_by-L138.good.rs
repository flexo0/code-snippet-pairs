fn issue16405() {
    items.sort_by_key(|item1| item1.value.clone());
}
