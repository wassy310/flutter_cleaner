use std::process::Command;
use std::fs;

fn main() {
    let project_name = std::env::args().nth(1).expect("Please provide a project name.");

    let output = Command::new("flutter.bat")
        .arg("create")
        .arg(&project_name)
        .output()
        .expect("Failed to create Flutter project");

    if !output.status.success() {
        panic!("Failed to create Flutter project");
    }

    remove_comments(&format!("{}/lib/main.dart", project_name));
    remove_comments(&format!("{}/pubspec.yaml", project_name));
}

fn remove_comments(file_path: &str) {
    let contents = fs::read_to_string(&file_path).expect("Failed to read file.");

    let filtered: Vec<String> = contents.lines()
        .filter(|line| !line.trim_start().starts_with("//") && !line.trim_start().starts_with("#"))
        .map(|s| s.to_string())
        .collect();

    fs::write(&file_path, filtered.join("\n")).expect("Failed to write file.");
}
