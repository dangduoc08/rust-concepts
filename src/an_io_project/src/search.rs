pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  let query: String = query.to_lowercase();
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}
