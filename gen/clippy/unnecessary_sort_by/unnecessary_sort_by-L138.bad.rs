fn issue16405() {
    items.sort_by(|item1, item2| item1.value.clone().cmp(&item2.value.clone()));
}
