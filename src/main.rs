use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use hello_rust2::{my_function, content::function_content, database, push_github}; // Import từ lib.rs


#[tokio::main]
async fn main() {
    // --- Phần kết nối MySQL ---
    let poem_content = database::get_poem_content().await;
    // Gọi `my_function()` để lấy nội dung cần ghi vào file
    let my_func_output = my_function(); // Lấy nội dung từ my_function()
    let my_func_content = function_content();

    // Tạo nội dung HTML
    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
    <pre>{poem_content}</pre>
    <p>{my_func_output}</p> <!-- Chèn kết quả của my_function() -->
    <p>{my_func_content}</p> <!-- Chèn kết quả của function_content() -->
  </body>
</html>"#
         
    );

    // Tạo thư mục front-end nếu chưa tồn tại
    if let Err(e) = fs::create_dir_all("./front-end").await {
        eprintln!("Lỗi tạo thư mục front-end: {:?}", e);
    }

    // Ghi nội dung vào file hello.html
    let mut file = File::create("./front-end/hello.html")
        .await
        .expect("Không thể tạo file hello.html");
    file.write_all(html_content.as_bytes())
        .await
        .expect("Lỗi khi ghi file hello.html");

    println!("✅ Đã tạo file hello.html!");

    // Đẩy code lên GitHub
    push_github::push_to_github();
}
