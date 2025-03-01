use warp::Filter;
use hello_rust2::*;
use hello_rust2::database;
use hello_rust2::back_end::{api, routes};
use tokio::signal;

#[tokio::main]
async fn main() {
    // 👉 Kết nối database
    let pool = database::connect_db().await.expect("Không thể kết nối MySQL");
    
    // 👉 Lấy dữ liệu từ database
    let poem_data = route::get_poem_data(&pool).await;

    // 👉 Khởi tạo các route cho server 8080
    let hello_route = route::create_html_route(poem_data.clone());
    let api_route = route::create_api_route(poem_data.clone());
    let login_route = route::create_login_route(pool.clone());
    let register_route = route::create_register_route(pool.clone());
    let static_files = route::create_static_route();

    // 👉 Áp dụng CORS cho route đăng nhập
    let call_login = login_route.with(warp::cors().allow_any_origin());

    println!("🚀 Server chạy tại http://localhost:8080/");

    // 👉 Chạy server trên cổng 8080 trong một task riêng
    let server_8080 = tokio::spawn(async move {
        warp::serve(hello_route.or(api_route).or(register_route).or(static_files).or(call_login))
            .run(([127, 0, 0, 1], 8080))
            .await;
    });

    let pools = api::connect_db().await.expect("Không thể kết nối MySQL");
    
    // 👉 Khởi tạo server 9090
    let db_pool = pools.clone();
    let api_routes = routes::routess(db_pool);

    println!("🚀 Server chạy tại http://localhost:9090/api/all");

    // 👉 Chạy server trên cổng 9090 trong một task riêng
    let server_9090 = tokio::spawn(async move {
        warp::serve(api_routes).run(([127, 0, 0, 1], 9090)).await;
    });

    // Bắt tín hiệu Ctrl+C để đẩy code lên GitHub
    let shutdown_signal = async {
        signal::ctrl_c().await.expect("Không thể bắt tín hiệu Ctrl+C");
        println!("📌 Nhận tín hiệu Ctrl+C, đang đẩy code lên GitHub...");
        hello_rust2::push_github::push_to_github(); // Gọi hàm đẩy GitHub
        println!("✅ Hoàn thành! Đang tắt server...");
    };

    // 👉 Chạy cả hai server cùng với tín hiệu dừng
    tokio::select! {
        _ = server_8080 => {},
        _ = server_9090 => {},
        _ = shutdown_signal => {
            println!("🔄 Đang tắt server...");
        },
    }
    
    println!("✅ Server đã dừng an toàn.");
}
