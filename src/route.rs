use warp::Filter;

use crate::front_end;
use sqlx::MySqlPool;

use tokio::signal;
use std::future::Future;

// 👉 Hàm lấy dữ liệu thơ từ database


// 👉 Route `/hello/{chapter_name}`
pub fn create_hello_route(
    pool: MySqlPool,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("hello" / String)
        .and(with_db(pool))
        .and_then(handle_hello)
}

// Middleware để truyền database pool vào handler
fn with_db(
    pool: MySqlPool,
) -> impl Filter<Extract = (MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

// 👉 Hàm xử lý `/hello/{chapter_name}`
async fn handle_hello(
    chapter_name: String,
    pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match front_end::hello::home(&pool, &chapter_name).await {
        Ok(html) => Ok(warp::reply::html(html)),
        Err(_) => Err(warp::reject::not_found()),
    }
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






