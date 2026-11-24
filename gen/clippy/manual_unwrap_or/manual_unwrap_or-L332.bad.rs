fn issue_15807() {
    let _ = if let Ok(v) = x { v } else { 2 };
}
