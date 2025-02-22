pub fn generate_html(poem_content: &str, my_func_content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello! My name is Bao</h1>
    <p>Hi from Rust</p>
    <pre>{}</pre>
    <p>{}</p> <!-- Chèn kết quả của function_content() -->
  </body>
</html>"#,
        poem_content, my_func_content
    )
}
