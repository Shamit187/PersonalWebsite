use std::env;
use std::fs;
use std::process;
use chrono::Local;

mod parser;
fn print_usage() {
    eprintln!("Usage: blog_writer <title> <author>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage();
        process::exit(1);
    }

    let title = &args[1];
    let author = &args[2];
    let path = "edit.md";

    let created_on = Local::now().format("%Y-%m-%d").to_string();

    let markdown_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading file: {}", path);
            process::exit(1);
        }
    };
    let background_image = match fs::read_to_string("background_image.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading file: {}", path);
            process::exit(1);
        }
    };

    let html_content = parser::markdown_to_html(&markdown_content, &background_image, title, author, &created_on);

    let output_path = "preview.html";
    match fs::write(&output_path, html_content) {
        Ok(_) => println!("File written to {}", output_path),
        Err(_) => {
            eprintln!("Error writing file: {}", output_path);
            process::exit(1);
        }
    }
}