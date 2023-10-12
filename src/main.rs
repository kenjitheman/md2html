extern crate pulldown_cmark;
extern crate std;

use pulldown_cmark::{html, Parser};
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    let mut input_path = String::new();
    println!("[+] enter the path to the markdown file: ");
    io::stdin()
        .read_line(&mut input_path)
        .expect("failed to read input");

    input_path = input_path.trim().to_string();

    let markdown_input = match fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("error reading the file: {}", err);
            return;
        }
    };

    let parser = Parser::new(&markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let input_path = PathBuf::from(&input_path);
    let mut html_output_path = input_path.clone();
    html_output_path.set_extension("html");

    if let Err(err) = fs::write(&html_output_path, &html_output) {
        eprintln!("error writing the HTML file: {}", err);
    } else {
        println!("[SUCCESS] HTML file created at: {:?}", html_output_path);
    }
}
