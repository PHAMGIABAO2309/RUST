
use sqlx::MySqlPool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterForm {
    username: String,
    password: String,
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
                <label for="password">Mật khẩu:</label>
                <input type="password" id="password" name="password" required>
                <br>
                <button type="submit">Đăng Ký</button>
            </form>
        </body>
    </html>
    "#.to_string()
}

// 👉 Xử lý đăng ký (POST /register)
pub async fn handle_register(pool: MySqlPool, form: RegisterForm) -> Result<impl warp::Reply, warp::Rejection> {
    let query = "INSERT INTO users (username, password) VALUES (?, ?)";
    
    match sqlx::query(query)
        .bind(&form.username)
        .bind(&form.password)
        .execute(&pool)
        .await 
    {
        Ok(_) => {
            let response = warp::reply::html(r#"
                <html>
                    <body>
                        <h3>Đăng ký thành công! Chuyển hướng...</h3>
                        <script>setTimeout(() => { window.location.href = "/hello"; }, 2000);</script>
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
