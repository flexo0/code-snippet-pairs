fn main() {
    sentence.trim_end_matches(|c: char| c == '.' || c == ',' || c == '!' || c == '?');
}
