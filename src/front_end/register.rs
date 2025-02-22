
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
        <head><title>Đăng Ký</title></head>
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

// Xử lý đăng ký tài khoản
pub async fn handle_register(pool: MySqlPool, form: RegisterForm) -> Result<impl warp::Reply, warp::Rejection> {
    // Kiểm tra tài khoản đã tồn tại chưa
    let existing_user = sqlx::query("SELECT username FROM users WHERE username = ?")
        .bind(&form.username)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if existing_user.is_some() {
        return Ok(warp::reply::html("<h3>Tên đăng nhập đã tồn tại! Thử tên khác.</h3>"));
    }

    // Thêm tài khoản mới vào database
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