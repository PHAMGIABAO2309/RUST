pub fn home(poem_content: &str, my_func_content: &str) -> String {
  format!(
      r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Hello from Rust!</title>
</head>
<body>
  <h1>Tôi là Phạm Gia Bảo! Tôi đang test Localhost</h1>
  <pre>{}</pre>
  <p>{}</p>
  <img class="home-image" src="/images/Home.jpg" alt="Home">
  <a href="/register">Đăng ký</a>
</body>
</html>"#,
      poem_content, my_func_content
  )
}
