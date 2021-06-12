use super::r#struct;
use std::env;

impl r#struct::Config {
  pub fn new<'a>(mut args: env::Args) -> Result<r#struct::Config, &'a str> {
    args.next();

    let query: String = match args.next() {
      Some(q) => q.to_string(),
      None => return Err("didn't get a query string"),
    };

    let filename: String = match args.next() {
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
