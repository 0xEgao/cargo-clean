use figlet_rs::FIGfont;
use std::{env, fs, path::Path, process::Command};
fn visit_dir(path: &Path) {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            println!("current dir: {:?}", entry_path);

            if entry_path.join("Cargo.toml").exists() {
                let command = Command::new("cargo")
                    .arg("clean")
                    .current_dir(&entry_path)
                    .output()
                    .expect("Failed to clean directory");
                if command.status.success() {
                    println!("Output : {:?}", command);
                } else {
                    println!("Failed to clean {:?}", entry_path);
                }
            } else {
                visit_dir(&entry_path);
            }
        }
    }
}

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Clean Cargo files ");
    println!("{}", figure.unwrap());
    visit_dir(&env::current_dir().unwrap().parent().unwrap());
}
