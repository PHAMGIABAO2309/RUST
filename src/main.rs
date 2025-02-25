use warp::Filter;
use hello_rust2::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let pool = database::connect_db().await.expect("Không thể kết nối MySQL");  // 👉 Kết nối database

    let chapter_name = "Chương 1"; // Tên chương để lấy nội dung thơ
    let poem_content = route::get_poem_data(&pool, chapter_name).await; // Lấy nội dung thơ từ database

    // 👉 Dữ liệu thơ cần có đủ 4 phần: chapter, title, rules, content
    let poem_data = Arc::new(Mutex::new(("Chương 1".to_string(), "Tiêu đề".to_string(), "Quy tắc".to_string(), poem_content.lock().await.clone())));

    // 👉 Cấu hình các routes
    let hello_route = route::create_hello_route(poem_data.clone()); // Fix kiểu dữ liệu
    let login_route = route::create_login_route(pool.clone());
    let call_login = login_route.with(warp::cors().allow_any_origin());
    let register_route = route::create_register_route(pool.clone());
    let static_files = route::create_static_route();

    println!("🚀 Server chạy tại http://localhost:8080/");
    let server = warp::serve(hello_route.or(register_route).or(static_files).or(call_login)) // 👉 Chạy server
        .run(([127, 0, 0, 1], 8080));
    route::wait_for_exit(server).await; // 👉 Chờ tín hiệu Ctrl+C và đẩy lên GitHub
}
