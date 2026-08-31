use std::path::Path;
use std::fs;
const TOOL: &str = "stack-trace-99b4d1";
fn scan(path: &Path) -> Vec<String> { let mut files = Vec::new(); if let Ok(entries) = fs::read_dir(path) { for entry in entries.flatten() { let name = entry.file_name().to_string_lossy().to_string(); if !name.starts_with('.') { files.push(name); } } } files.sort(); files }
fn main() { println!("[{}] Scanning current directory...", TOOL); let files = scan(Path::new(".")); for f in &files { println!("  {}", f); } println!("[{}] Total: {} entries", TOOL, files.len()); }
