use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    // Nội dung HTML
    let html_content = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust </p>cc
    <p> Tôi tên là Bảo Phạm Gia Sang Nguyen lông phi hahahaha  êre cc  Huy</p>
  </body>
</html>"#;

    // Tạo file hello.html
    let mut file = File::create("./front-end/hello.html").expect("Không thể tạo file!");
    file.write_all(html_content.as_bytes()).expect("Lỗi khi ghi file!");

    println!("✅ Đã tạo file hello.html!");

    // Chạy http-server bằng Node.js
    let _ = Command::new("cmd")
        .args(["/C", "http-server -p 8080"])
        .spawn()
        .expect("Không thể chạy server!");

    println!("🚀 Server chạy tại http://localhost:8080");
}
