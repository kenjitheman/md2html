## Features
- Converts Markdown files to HTML.
- Provides a command-line interface for easy conversion.
- Handles basic Markdown formatting.
- It uses the `pulldown-cmark` library for parsing Markdown and generating HTML.


## Installation

```sh
git clone https://github.com/kenjitheman/md-to-html
```

```sh
cargo build --release
```

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
