use warp::Filter;
use hello_rust2::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let pool = database::connect_db().await.expect("Không thể kết nối MySQL");  // 👉 Kết nối database 
    let poem_content = route::get_poem_data(&pool, "NGHỊ ĐỊNH").await;   // 👉 Lấy dữ liệu thơ từ database
    // 👉 Chuyển đổi dữ liệu để phù hợp với `create_hello_route`
    let poem = Arc::new(Mutex::new((
        "NGHỊ ĐỊNH".to_string(),
        "".to_string(),
        "".to_string(),
        poem_content.lock().await.clone(),
    )));

    // 👉 Cấu hình các routes
    let hello_route = route::create_hello_route(poem.clone());
    let login_route = route::create_login_route(pool.clone());
    let call_login = login_route.with(warp::cors().allow_any_origin());
    let register_route = route::create_register_route(pool.clone());
    let static_files = route::create_static_route();

    println!("🚀 Server chạy tại http://localhost:8080/");
    let server = warp::serve(hello_route.or(register_route).or(static_files).or(call_login)) // 👉 Chạy server
        .run(([127, 0, 0, 1], 8080));
        
    route::wait_for_exit(server).await; // 👉 Chờ tín hiệu Ctrl+C
}