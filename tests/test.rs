use pulldown_cmark::Parser;
use pulldown_cmark::html;

#[test]
fn test_markdown_to_html() {
    let markdown_content = "Hello, **Markdown**!";
    let expected_html = "<p>Hello, <strong>Markdown</strong>!</p>\n";

    let parser = Parser::new(markdown_content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    assert_eq!(html_output, expected_html);
}
