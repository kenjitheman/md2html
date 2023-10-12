## markdown to html converter

<div align="center">
  <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-plain.svg" height="200" alt="rust logo"  />
</div>


## project structure:

```rust
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── src
│   └── main.rs
└── tests
    └── test.rs
```

- a simple command-line tool in Rust to convert Markdown files to HTML
- this project is a Markdown to HTML converter written in rust
- it provides a command-line interface for converting Markdown files to HTML files 
- it uses the `pulldown-cmark` library for parsing Markdown and generating HTML


## features

- converts Markdown files to HTML
- provides a command-line interface for easy conversion
- handles basic Markdown formatting


## installation

- 1

```sh
git clone https://github.com/kenjitheman/md-to-html
```

- 2

```sh
cargo build --release
```

- 3

```sh
cargo run --bin mdtohtml
```

## example

```sh
cargo run --bin markdown_to_html_converter
[+] enter the path to the Markdown file: example.md
[SUCCESS] HTML file created at: example.html
```

## contributing

- pull requests are welcome, for major changes, please open an issue first to
  discuss what you would like to change

- please make sure to update tests as appropriate

## license

- [MIT](https://choosealicense.com/licenses/mit/)
