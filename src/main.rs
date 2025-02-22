use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use hello_rust2::*; // Import từ lib.rs
include!("../front-end/hello.rs");

#[tokio::main]
async fn main() {
    let poem_content = database::get_poem_content().await; // --- Phần kết nối MySQL ---
    let my_func_content = content::function_content(); // Gọi `my_function()` để lấy nội dung cần ghi vào file 
    // Gọi hàm từ file hello.rs
    let html_content = generate_html(&poem_content, &my_func_content);

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

    
    push_github::push_to_github(); // Đẩy code lên GitHub
}
