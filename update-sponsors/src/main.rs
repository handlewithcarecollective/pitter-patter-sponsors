use std::{fs::{self, read_dir}, path::{Path, PathBuf}};

use clap::Parser;
use regex::Regex;
use std::fs::read_to_string;

const LOGO_URL_BASE: &'static str = "https://media.githubusercontent.com/media/handlewithcarecollective/pitter-patter-sponsors/main/logos/";

const LOGO_BLOCK_OPEN: &'static str = "<!-- Start Sponsor Block 25dfc72f-3e1c-499d-a968-12c49dbaa9b5 -->";
const LOGO_BLOCK_CLOSE: &'static str = "<!-- Close Sponsor Block 25dfc72f-3e1c-499d-a968-12c49dbaa9b5 -->";


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
  save_readme(&config.readme_path, &updated_readme_lines);
}

fn get_logo_file_names(logo_dir: &Path) -> Vec<String> {
  read_dir(logo_dir).unwrap()
  .map(|x| x.unwrap().file_name().to_str().unwrap().to_string())
  .collect::<Vec<_>>()
}

fn generate_logo_block(logo_file_names: &[String]) -> String {
  logo_file_names.iter()
  .map(|file_name| format!("![file_name]({LOGO_URL_BASE}{file_name})"))
  .collect::<Vec<_>>()
  .join("\n")
}

fn load_readme(readme_path: &Path) -> String {
  read_to_string(readme_path).unwrap()
}

fn save_readme(readme_path: &Path, readme_str: &str) {
  fs::write(readme_path, readme_str).unwrap();
}

fn update_readme_logo_block(readme_lines: &str, logo_block: &str) -> String {
  let re = Regex::new(&format!(r"(?s){}.*{}", regex::escape(LOGO_BLOCK_OPEN), regex::escape(LOGO_BLOCK_CLOSE))).unwrap();
  let full_logo_block = format!("{LOGO_BLOCK_OPEN}\n{logo_block}\n{LOGO_BLOCK_CLOSE}");

  if !re.is_match(readme_lines) {
    panic!("Did not find logo block in readme.");
  }

  let updated_readme = re.replace(readme_lines, full_logo_block);
  let updated_readme = String::from(updated_readme);
  
  updated_readme
}