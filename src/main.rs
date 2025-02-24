use warp::Filter;
use hello_rust2::*;

#[tokio::main]
async fn main() {
    let pool = database::connect_db().await.expect("Không thể kết nối MySQL");  // 👉 Kết nối database
    let _poem_content = route::get_poem_data(&pool).await;   // 👉 Lấy dữ liệu thơ từ database (nhưng không cần đưa vào route)

    // 👉 Cấu hình các routes
    let hello_route = route::create_hello_route(pool.clone()); // ✅ Dùng pool, không dùng poem_content
    let login_route = route::create_login_route(pool.clone());
    let register_route = route::create_register_route(pool.clone());
    let static_files = route::create_static_route();

    let routes = hello_route
        .or(login_route.with(warp::cors().allow_any_origin())) // ✅ Cho phép CORS với route login
        .or(register_route)
        .or(static_files);

    println!("🚀 Server chạy tại http://localhost:8080/");

    let server = warp::serve(routes).run(([127, 0, 0, 1], 8080));
    route::wait_for_exit(server).await; // 👉 Chờ Ctrl+C và đẩy lên GitHub
}
