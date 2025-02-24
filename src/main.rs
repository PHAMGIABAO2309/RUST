use warp::Filter;
use hello_rust2::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::mysql::MySqlPool; // 👉 Import Pool MySQL

#[tokio::main]
async fn main() {
    let pool = database::connect_db().await.expect("Không thể kết nối MySQL");

    // 👉 Kết nối database
   

    // 👉 Lấy dữ liệu thơ từ database
    let poem_content = get_poem_data(&pool).await;

    // 👉 Lấy nội dung function_content()
    let my_func_content = Arc::new(content::function_content());

    // 👉 Cấu hình các routes
    let hello_route = create_hello_route(poem_content.clone(), my_func_content.clone());
    let register_route = create_register_route(pool.clone());
    let static_files = create_static_route();

    println!("🚀 Server chạy tại http://localhost:8080/");

    // 👉 Chạy server và đợi tín hiệu Ctrl+C
    let server = warp::serve(hello_route.or(register_route).or(static_files))
        .run(([127, 0, 0, 1], 8080));

    wait_for_exit(server).await;
}

// 👉 Hàm kết nối database


// 👉 Hàm lấy dữ liệu thơ từ database
// 👉 Hàm lấy dữ liệu thơ từ database
async fn get_poem_data(conn: &MySqlPool) -> Arc<Mutex<String>> {
    let poem_content = match database::get_poem_content(conn).await {
        Ok(content) => Arc::new(Mutex::new(content)), // Nếu dữ liệu có thể thay đổi
        Err(_) => Arc::new(Mutex::new("Không thể lấy dữ liệu thơ".to_string())),
    };
    poem_content
}


// 👉 Hàm tạo route `/hello`
fn create_hello_route(poem: Arc<Mutex<String>>, func: Arc<String>) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("hello").and_then(move || handle_hello(poem.clone(), func.clone()))
}

// 👉 Hàm tạo route `/register`
fn create_register_route(pool: MySqlPool) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("register")
        .and(warp::get())
        .map(|| warp::reply::html(front_end::register::register_page()))
        .or(warp::path("register")
            .and(warp::post())
            .and(warp::body::form())
            .and_then(move |form_data| {
                let pool_clone = pool.clone();
                async move { front_end::register::handle_register(pool_clone, form_data).await }
            }))
}

// 👉 Hàm tạo route file tĩnh
fn create_static_route() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("static").and(warp::fs::dir("./static"))
}

// 👉 Hàm xử lý request `/hello`
async fn handle_hello(poem: Arc<Mutex<String>>, func: Arc<String>) -> Result<impl warp::Reply, warp::Rejection> {
    let poem_content = poem.lock().await.clone();
    let html = front_end::hello::home(&poem_content, &func);
    Ok(warp::reply::html(html))
}

// 👉 Hàm đợi tín hiệu Ctrl+C để dừng server
async fn wait_for_exit(server: impl std::future::Future<Output = ()>) {
    tokio::select! {
        _ = server => {},
        _ = tokio::signal::ctrl_c() => {
            println!("📌 Nhận tín hiệu Ctrl+C, đẩy code lên GitHub...");
            push_github::push_to_github();
        }
    }
}
