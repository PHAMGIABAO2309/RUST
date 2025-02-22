use warp::Filter;
use hello_rust2::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Kết nối database
    let pool = database::connect_db().await.expect("Không thể kết nối MySQL");

    // Lấy dữ liệu thơ từ database
    let poem_content = match database::get_poem_content(&pool).await {
        Ok(content) => Arc::new(Mutex::new(content)),  // Nếu dữ liệu có thể thay đổi
        Err(_) => Arc::new(Mutex::new("Không thể lấy dữ liệu thơ".to_string())),
    };

    // Lấy nội dung function_content() (không thay đổi nên không cần Mutex)
    let my_func_content = Arc::new(content::function_content());

    // 👉 Route trang `/hello`
    let hello_route = warp::path("hello").and_then(move || handle_hello(poem_content.clone(), my_func_content.clone()));

    // 👉 Route trang `/register`
    let pool_clone = pool.clone();
    let register_route = warp::path("register")
        .and(warp::get())
        .map(|| warp::reply::html(front_end::register::register_page()))
        .or(warp::path("register")
            .and(warp::post())
            .and(warp::body::form())
            .and_then(move |form_data| {
                let pool_clone = pool_clone.clone();
                async move { front_end::register::handle_register(pool_clone, form_data).await }
            }));

    // Route file tĩnh (CSS, JS, hình ảnh)
    let static_files = warp::path("static").and(warp::fs::dir("./static"));

    println!("🚀 Server chạy tại http://localhost:8080/");

    // Chạy server và đợi tín hiệu Ctrl+C
    let server = warp::serve(hello_route.or(register_route).or(static_files))
        .run(([127, 0, 0, 1], 8080));

    tokio::select! {
        _ = server => {},
        _ = tokio::signal::ctrl_c() => {
            println!("📌 Nhận tín hiệu Ctrl+C, đẩy code lên GitHub...");
            push_github::push_to_github();
        }
    }
}

// 👉 Hàm xử lý request `/hello`
async fn handle_hello(poem: Arc<Mutex<String>>, func: Arc<String>) -> Result<impl warp::Reply, warp::Rejection> {
    let poem_content = poem.lock().await.clone();  // Lấy nội dung thơ
    let html = front_end::hello::home(&poem_content, &func);
    Ok(warp::reply::html(html))
}


