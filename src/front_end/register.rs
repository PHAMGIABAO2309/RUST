use sqlx::MySqlPool;
use serde::Deserialize;
use warp::{Rejection, Reply};
#[derive(Deserialize)]
pub struct RegisterForm {
    username: String,
    password: String,
    email: String,
}

// Hàm kiểm tra username đã tồn tại chưa
async fn username_exists(pool: &MySqlPool, username: &str) -> Result<bool, sqlx::Error> {
    let query = "SELECT COUNT(*) FROM users WHERE username = ?";
    let count: (i64,) = sqlx::query_as(query)
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(count.0 > 0)
}

// 👉 Xử lý đăng ký nhiều tài khoản (POST /register)
pub async fn handle_register(pool: MySqlPool, form: RegisterForm) -> Result<impl Reply, Rejection> {
    if username_exists(&pool, &form.username).await.unwrap_or(false) {
        let response = warp::reply::html("<h3>Tên đăng nhập đã tồn tại, vui lòng chọn tên khác!</h3>");
        return Ok(response);
    }

    let query = "INSERT INTO users (username, password, email) VALUES (?, ?, ?)";

    match sqlx::query(query)
        .bind(&form.username)
        .bind(&form.password)
        .bind(&form.email)
        .execute(&pool)
        .await 
    {
        Ok(_) => {
            let response = warp::reply::html(r#"
                <html>
                    <body>
                        <h3>Đăng ký thành công! Chuyển hướng...</h3>
                        <script>window.location.href = "/hello";</script>
                    </body>
                </html>
            "#);
            Ok(response)
        }
        Err(e) => {
            eprintln!("Lỗi khi đăng ký: {:?}", e);
            let response = warp::reply::html("<h3>Đăng ký thất bại, thử lại!</h3>");
            Ok(response)
        }
    }
}


// 👉 Hiển thị trang đăng ký (GET /register)
pub fn register_page() -> String {
    r#"
    <html>
        <head>
        <meta charset="utf-8">
        <title>Đăng Ký</title>
        <link rel="stylesheet" href="/static/register.css">
        </head>
        <body>
            <h2>Trang Đăng Ký</h2>
            <form action="/register" method="post">
                <label for="username">Tên đăng nhập:</label>
                <input type="text" id="username" name="username" required>
                <br>
                <label for="email">Email:</label>
                <input type="email" id="email" name="email" required>
                <br>
                <label for="password">Mật khẩu:</label>
                <input type="password" id="password" name="password" required>
                <br>
                
                <button type="submit">Đăng Ký</button>
            </form>
        </body>
    </html>
    "#.to_string()
}


