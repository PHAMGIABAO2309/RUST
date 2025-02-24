use warp::Filter;
use hello_rust2::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let pool = database::connect_db().await.expect("Không thể kết nối MySQL");  // 👉 Kết nối database

    // 👉 Lấy dữ liệu thơ từ database
    let poem_content = route::get_poem_data(&pool).await;

    // 👉 Lấy nội dung function_content()
    let my_func_content = Arc::new(content::function_content());

    // 👉 Cấu hình các routes
    let hello_route = route::create_hello_route(poem_content.clone(), my_func_content.clone());
    let register_route = route::create_register_route(pool.clone());
    let static_files = route::create_static_route();

    println!("🚀 Server chạy tại http://localhost:8080/");

    let server = warp::serve(hello_route.or(register_route).or(static_files)) // 👉 Chạy server
        .run(([127, 0, 0, 1], 8080));

    route::wait_for_exit(server).await;
}



