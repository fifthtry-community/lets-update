pub fn post<T: std::fmt::Display>(guid: T) -> String {
    format!("/u/{guid}/")
}
