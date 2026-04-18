use std::{fs::{self, read_dir}, path::{Path, PathBuf}};

use clap::Parser;
use std::fs::read_to_string;

const LOGO_URL_BASE: &'static str = "https://media.githubusercontent.com/media/handlewithcarecollective/pitter-patter-sponsors/main/logos/";

const LOGO_BLOCK_OPEN: &'static str = "<!-- Start Sponsor Block 25dfc72f-3e1c-499d-a968-12c49dbaa9b5 -->";
const LOGO_BLOCK_CLOSE: &'static str = "<!-- Close Sponsor Block 25dfc72f-3e1c-499d-a968-12c49dbaa9b5 -->";


#[derive(Parser, Clone, Debug)]
pub struct Config {
  #[arg(long = "logo_dir")]
  pub logo_dir: PathBuf,
  #[arg(long = "readme_path")]
  pub readme_path: PathBuf,
}

fn main() {
  let config = Config::parse();

  // read logo files
  let logo_file_names = get_logo_file_names(&config.logo_dir);

  // create logo block
  let logo_block = generate_logo_block(&logo_file_names);

  // load readme as a vec of lines
  let readme_lines = load_readme(&config.readme_path);

  // update readme with new logo block
  let updated_readme_lines = update_readme_logo_block(&readme_lines, &logo_block);

  // save readme
  save_readme(&config.readme_path, &updated_readme_lines);
}

fn get_logo_file_names(logo_dir: &Path) -> Vec<String> {
  read_dir(logo_dir).unwrap()
  .map(|x| x.unwrap().file_name().to_str().unwrap().to_string())
  .collect::<Vec<_>>()
}

fn generate_logo_block(logo_file_names: &[String]) -> Vec<String> {
  logo_file_names.iter()
  .map(|file_name| format!("![file_name]({LOGO_URL_BASE}{file_name})"))
  .collect::<Vec<_>>()
}

fn load_readme(readme_path: &Path) -> Vec<String> {
  let mut result = Vec::new();
  for line in read_to_string(readme_path).unwrap().lines() {
    result.push(line.to_string())
  }
  result
}

fn save_readme(readme_path: &Path, readme_lines: &[String]) {
  let readme_str = readme_lines.join("\n");
  fs::write(readme_path, readme_str).unwrap();
}

fn update_readme_logo_block(readme_lines: &[String], logo_block: &[String]) -> Vec<String> {
  let mut updated_readme = vec![];
  let mut line_iter = readme_lines.iter().peekable();
  
  let mut open_block_found = false;
  let mut close_block_found = false;
  while let Some(next_line) = line_iter.next() {
    updated_readme.push(next_line.to_string());
    if next_line == LOGO_BLOCK_OPEN {
      open_block_found = true;
      break;
    }
  }

  for next_line in logo_block.iter() {
    updated_readme.push(next_line.to_string());
  }

  while let Some(next_line) = line_iter.peek() {
    if *next_line == LOGO_BLOCK_CLOSE {
      close_block_found = true;
      break;
    }
    line_iter.next();
  }

  for next_line in line_iter {
    updated_readme.push(next_line.to_string());
  }

  if !open_block_found {
    panic!("Did not find logo block open in readme. Line must exactly match \"{LOGO_BLOCK_OPEN}\"");
  }

  if !close_block_found {
    panic!("Did not find logo block close in readme. Line must exactly match \"{LOGO_BLOCK_CLOSE}\"");
  }
  
  updated_readme
}