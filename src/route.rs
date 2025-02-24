use warp::Filter;
use crate::database::get_poem_content;
use crate::front_end; // Không dùng hello_rust2::front_end
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::mysql::MySqlPool;



pub async fn get_poem_data(conn: &MySqlPool) -> Arc<Mutex<String>> {
    let poem_content = match get_poem_content(conn).await {
        Ok(content) => Arc::new(Mutex::new(content)),
        Err(_) => Arc::new(Mutex::new("Không thể lấy dữ liệu thơ".to_string())),
    };
    poem_content
}
// 👉 Route `/hello`
pub fn create_hello_route(
    poem: Arc<Mutex<String>>, 
    func: Arc<String>
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("hello").and_then(move || handle_hello(poem.clone(), func.clone()))
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

// 👉 Route file tĩnh `/static`
pub fn create_static_route() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("static").and(warp::fs::dir("./static"))
}

// 👉 Hàm xử lý `/hello`
async fn handle_hello(
    poem: Arc<Mutex<String>>, 
    func: Arc<String>
) -> Result<impl warp::Reply, warp::Rejection> {
    let poem_content = poem.lock().await.clone();
    let html = front_end::hello::home(&poem_content, &func);
    Ok(warp::reply::html(html))
}

