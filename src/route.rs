use warp::Filter;
use crate::front_end; // Không dùng hello_rust2::front_end
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::mysql::MySqlPool;
use tokio::signal;
use std::future::Future;
use serde_json::Value;

// 📌 Route trả về API JSON
// 📌 API JSON: Trả về nội dung dưới dạng JSON
pub fn create_api_route(
    poem: Arc<Mutex<serde_json::Value>>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(warp::path("content"))
        .and(warp::get())
        .and_then(move || {
            let poem_clone = poem.clone();
            async move {
                let json_response = poem_clone.lock().await.clone();
                Ok::<_, warp::Rejection>(warp::reply::json(&json_response))
            }
        })
}

// 📌 Route HTML: Hiển thị nội dung trên trình duyệt
pub fn create_html_route(
    poem: Arc<Mutex<serde_json::Value>>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("hello").and_then(move || handle_hello(poem.clone()))
}

// 👉 Hàm xử lý HTML `/hello`
async fn handle_hello(
    _poem: Arc<Mutex<serde_json::Value>>, // Không cần dùng biến này nữa
) -> Result<impl warp::Reply, warp::Rejection> {
    let html = front_end::hello::home(); // Gọi không truyền tham số
    Ok(warp::reply::html(html))
}


pub async fn get_poem_data(conn: &MySqlPool) -> Arc<Mutex<Value>> {
    match front_end::content::get_document_content(conn).await {
        Ok(content) => Arc::new(Mutex::new(content)),  // Giữ nguyên kiểu JSON
        Err(_) => Arc::new(Mutex::new(serde_json::json!({ "error": "Không thể lấy dữ liệu" }))),
    }
}








// 👉 Route `/register`
pub fn create_register_route(
    conn: MySqlPool
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("register")
        .and(warp::get())
        .map(|| warp::reply::html(front_end::register::register_page()))
        .or(warp::path("register")
            .and(warp::post())
            .and(warp::body::form())
            .and_then(move |form_data| {
                let conn_clone = conn.clone();
                async move { front_end::register::handle_register(conn_clone, form_data).await }
            }))
}
pub fn create_login_route(
    conn: MySqlPool
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::get())
        .map(|| warp::reply::html(front_end::login::login_page())) // Hiển thị trang đăng nhập
        .or(warp::path("login")
            .and(warp::post())
            .and(warp::body::form())
            .and_then(move |form_data| {
                let conn_clone = conn.clone();
                async move { front_end::login::handle_login(conn_clone, form_data).await }
            }))
}

// 👉 Route file tĩnh `/static`
pub fn create_static_route() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let static_files = warp::path("static").and(warp::fs::dir("./static"));
    let image_files = warp::path("images").and(warp::fs::dir("./images"));
    static_files.or(image_files)
}




pub async fn wait_for_exit(server: impl Future<Output = ()>) {
    tokio::select! {
        _ = server => {},
        _ = signal::ctrl_c() => {
            println!("📌 Nhận tín hiệu Ctrl+C, đẩy code lên GitHub...");
            crate::push_github::push_to_github();
        }
    }
}