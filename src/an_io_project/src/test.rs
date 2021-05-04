#[cfg(test)]
pub mod tests {
  use crate::search;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search::search(query, contents));
  }

  fn case_insensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search::search_case_insensitive(query, contents));
  }
}
