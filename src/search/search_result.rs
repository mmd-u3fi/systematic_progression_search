#[derive(Debug)]
pub enum SearchResult {
    Unsolvable,
    Solved(Vec<String>)
}