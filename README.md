## Markdown to html converter

<div align="center">
  <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-plain.svg" height="200" alt="rust logo"  />
</div>


## Project structure:

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

- A simple command-line tool in Rust to convert Markdown files to HTML.
- This project is a Markdown to HTML converter written in rust.
- It provides a command-line interface for converting Markdown files to HTML files.
- It uses the `pulldown-cmark` library for parsing Markdown and generating HTML.


## Features

- Converts Markdown files to HTML.
- Provides a command-line interface for easy conversion.
- Handles basic Markdown formatting.


## Installation

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

## Example

```sh
cargo run --bin markdown_to_html_converter
[+] enter the path to the Markdown file: example.md
[SUCCESS] HTML file created at: example.html
```

## Contributing

- Pull requests are welcome, for major changes, please open an issue first to
  discuss what you would like to change.

- Please make sure to update tests as appropriate.

## License

- [MIT](https://choosealicense.com/licenses/mit/)
