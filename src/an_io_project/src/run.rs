use crate::config::r#struct;
use crate::search::*;
use std::error::Error;
use std::fs;

pub fn run(cfg: r#struct::Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(cfg.filename)?;

  let results: Vec<&str> = if cfg.is_case_sensitive {    
    search(&cfg.query, &content)
  } else {
    search_case_insensitive(&cfg.query, &content)
  };
  println!("=======Result=======");

  if results.len() == 0 {
    println!("Not found, try search with 'CASE_INSENSITIVE=1'");
  };

  for line in results {
    println!("{}", line);
  };

  Ok(())
}
