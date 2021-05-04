use minigrep::{config::r#struct, run};
use std::{env, process};

fn main() {
  let args: Vec<String> = env::args().collect();
  let cfg: r#struct::Config = r#struct::Config::new(args).unwrap_or_else(|err| {
    eprintln!("problem parsing arguments: {}", err);

    process::exit(1);
  });
  println!("Search for '{}'", cfg.query);

  if let Err(e) = run::run(cfg) {
    eprintln!("application error: {}", e);

    process::exit(1);
  }
}
