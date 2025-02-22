use warp::Filter;
use hello_rust2::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // 🛠 Kết nối database một lần và dùng lại
    let pool = database::connect_db().await.expect("Không thể kết nối tới MySQL");

    // 🛠 Lấy dữ liệu thơ từ database
    let poem_content = match database::get_poem_content(&pool).await {
        Ok(content) => Arc::new(Mutex::new(content)),
        Err(_) => Arc::new(Mutex::new("Không thể lấy dữ liệu thơ".to_string())),
    };

    // 🛠 Lấy nội dung function_content() (không cần Mutex vì nó không thay đổi)
    let my_func_content = Arc::new(content::function_content());

    // 🛠 Tạo route `/hello`
    let poem_clone = Arc::clone(&poem_content);
    let func_clone = Arc::clone(&my_func_content);
    
    let hello_route = warp::path("hello").and_then(move || {
        let poem_clone = Arc::clone(&poem_clone);
        let func_clone = Arc::clone(&func_clone);

        async move {
            let poem = poem_clone.lock().await.clone();
            let html = front_end::hello::generate_html(&poem, &func_clone);
            Ok::<_, warp::Rejection>(warp::reply::html(html))
        }
    });

    // 🛠 Route để phục vụ file tĩnh (CSS, JS, hình ảnh)
    let static_files = warp::path("static").and(warp::fs::dir("./static"));

    println!("🚀 Server chạy tại http://localhost:8080/hello");

    // 🔥 Chạy server và đợi tín hiệu Ctrl+C
    let server = warp::serve(hello_route.or(static_files))
        .run(([127, 0, 0, 1], 8080));

    tokio::select! {
        _ = server => {},  // Chạy server
        _ = tokio::signal::ctrl_c() => {
            println!("📌 Nhận tín hiệu Ctrl+C, đẩy code lên GitHub...");
            push_github::push_to_github();
        }
    }
}
