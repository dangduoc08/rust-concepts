
use std::env;
use super::r#struct;

impl r#struct::Config {
  pub fn new<'a>(args: Vec<String>) -> Result<r#struct::Config, &'a str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query: String = match args.get(1) {
      Some(q) => q.to_string(),
      None => return Err("didn't get a query string"),
    };

    let filename: String = match args.get(2) {
      Some(f) => f.to_string(),
      None => return Err("didn't get a query string"),
    };

    let is_case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

    Ok(r#struct::Config {
      query,
      filename,
      is_case_sensitive,
    })
  }
}
