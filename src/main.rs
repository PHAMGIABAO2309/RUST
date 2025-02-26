use warp::Filter;
use hello_rust2::*;
use hello_rust2::database;
use crate::route:: create_html_route;

#[tokio::main]
async fn main() {
    // 👉 Kết nối database
    let pool = database::connect_db()
        .await
        .expect("Không thể kết nối MySQL");

    // 👉 Lấy dữ liệu từ database
    let poem_data = route::get_poem_data(&pool).await;

    // 👉 Khởi tạo các route
    let hello_route = create_html_route(poem_data.clone());
    let api_route = route::create_api_route(poem_data.clone());
    let login_route = route::create_login_route(pool.clone());
    let register_route = route::create_register_route(pool.clone());
    let static_files = route::create_static_route();

    // 👉 Áp dụng CORS cho route đăng nhập
    let call_login = login_route.with(warp::cors().allow_any_origin());

    println!("🚀 Server chạy tại http://localhost:8080/");

    // 👉 Chạy server với các route
    let server = warp::serve(
        hello_route
            .or(api_route)
            .or(register_route)
            .or(static_files)
            .or(call_login),
    )
    .run(([127, 0, 0, 1], 8080));

    // 👉 Chờ tín hiệu Ctrl+C để tắt server an toàn
    route::wait_for_exit(server).await;
}