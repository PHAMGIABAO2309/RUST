use std::process::Command;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use sqlx::{MySql, Pool};
use hello_rust2::{my_function, content::function_content}; // Import từ lib.rs



/// Hàm tự động đẩy code lên GitHub
fn push_to_github() {
    // Thêm tất cả file đã thay đổi
    let status = Command::new("git")
        .args(&["add", "."])
        .status()
        .expect("Không thể chạy lệnh git add");
    if !status.success() {
        eprintln!("Lỗi: git add không thành công");
        return;
    }

    // Commit thay đổi với commit message tự động
    let commit_message = "Auto commit from cargo run";
    let status = Command::new("git")
        .args(&["commit", "-m", commit_message])
        .status()
        .expect("Không thể chạy lệnh git commit");
    if !status.success() {
        // Lưu ý: Có thể không có thay đổi để commit, điều này không cần coi là lỗi nghiêm trọng.
        eprintln!("Lưu ý: git commit không thực hiện được (có thể không có thay đổi mới)");
    }

    // Push lên remote (ở đây giả sử branch chính là 'main')
    let status = Command::new("git")
        .args(&["push", "origin", "master"])
        .status()
        .expect("Không thể chạy lệnh git push");
    if !status.success() {
        eprintln!("Lỗi: git push không thành công");
    } else {
        println!("✅ Đã tự động đẩy code lên GitHub thành công!");
    }
}

#[tokio::main]
async fn main() {
    // --- Phần kết nối MySQL ---
    let db_url = "mysql://root@localhost:3366/my_database"; 
    let pool = Pool::<MySql>::connect(db_url)
        .await
        .expect("Không thể kết nối tới MySQL");

    // Lấy nội dung bài thơ từ database
    let row: (String,) = sqlx::query_as("SELECT content FROM poems LIMIT 1")
        .fetch_one(&pool)
        .await
        .expect("Không thể lấy dữ liệu từ MySQL");
    let poem_content = row.0;

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
    push_to_github();
}
