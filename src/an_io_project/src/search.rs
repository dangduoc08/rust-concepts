pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&'a str> = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    };
  }

  result
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  let query: String = query.to_lowercase();
  let mut result: Vec<&'a str> = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    };
  }

  result
}
