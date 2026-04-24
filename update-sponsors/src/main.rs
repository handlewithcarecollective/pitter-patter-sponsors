use std::{fs::{self}, path::{PathBuf}};

use clap::Parser;
use regex::Regex;
use std::fs::read_to_string;

const LOGO_BLOCK_OPEN: &'static str = "<!--sponsorsstart-->";
const LOGO_BLOCK_CLOSE: &'static str = "<!--sponsorsend-->";

#[derive(Parser, Clone, Debug)]
pub struct Config {
  #[arg(long = "logo_block_path")]
  pub logo_block_path: PathBuf,
  #[arg(long = "readme_path")]
  pub readme_path: PathBuf,
}

fn main() {
  let config = Config::parse();

  // create logo block
  let logo_block = read_to_string(&config.logo_block_path).unwrap();

  // load readme as a vec of lines
  let readme = read_to_string(&config.readme_path).unwrap();

  // update readme with new logo block
  let updated_readme_lines = update_readme_logo_block(&readme, &logo_block);

  // save readme
  fs::write(config.readme_path, updated_readme_lines).unwrap();
}

fn update_readme_logo_block(readme_lines: &str, logo_block: &str) -> String {
  let re = Regex::new(&format!(r"(?s){}.*{}", regex::escape(LOGO_BLOCK_OPEN), regex::escape(LOGO_BLOCK_CLOSE))).unwrap();
  let full_logo_block = format!("{LOGO_BLOCK_OPEN}\n\n{logo_block}\n{LOGO_BLOCK_CLOSE}");

  if !re.is_match(readme_lines) {
    panic!("Did not find logo block in readme.");
  }

  let updated_readme = re.replace(readme_lines, full_logo_block);
  let updated_readme = String::from(updated_readme);

  updated_readme
}
