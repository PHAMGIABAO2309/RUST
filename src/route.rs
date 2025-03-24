use warp::Filter;
use crate::front_end;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::mysql::MySqlPool;
use serde_json::Value;



pub async fn get_poem_data(conn: &MySqlPool) -> Arc<Mutex<Value>> {
    match front_end::query_sql::get_sql(conn).await {
        Ok(content) => Arc::new(Mutex::new(content)),  // Giữ nguyên kiểu JSON
        Err(_) => Arc::new(Mutex::new(serde_json::json!({ "error": "Không thể lấy dữ liệu" }))),
    }
}
// 📌 API JSON: Trả về nội dung dưới dạng JSON
pub fn create_api_route(
    poem: Arc<Mutex<serde_json::Value>>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("content")
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
    warp::path("nghidinh").map(move || {
        let _poem = poem.lock();
        warp::reply::html(front_end::nghidinh::home())
    })
}



pub fn create_html_route_home(
    poem: Arc<Mutex<serde_json::Value>>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("trangchu").map(move || {
        let _poem = poem.lock(); // Giữ lại biến nhưng không dùng
        warp::reply::html(front_end::home::homemain())
    })
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
    let handle_files = warp::path("handle").and(warp::fs::dir("./handle"));
    static_files.or(image_files).or(handle_files)
}