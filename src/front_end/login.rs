use sqlx::MySqlPool;
use serde::Deserialize;
use warp::{Rejection, Reply};
use warp::reply::json;
use serde_json::json;

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

// Hàm kiểm tra thông tin đăng nhập
async fn check_credentials(pool: &MySqlPool, username: &str, password: &str) -> Result<bool, sqlx::Error> {
    let query = "SELECT COUNT(*) FROM account WHERE Username = ? AND Password = ?";
    let count: (i64,) = sqlx::query_as(query)
        .bind(username)
        .bind(password)
        .fetch_one(pool)
        .await?;
    Ok(count.0 > 0)
}

// 👉 Xử lý đăng nhập (POST /login)
pub async fn handle_login(pool: MySqlPool, form: LoginForm) -> Result<impl Reply, Rejection> {
    if check_credentials(&pool, &form.username, &form.password).await.unwrap_or(false) {
        Ok(json(&json!({ "success": true, "message": "Đăng nhập thành công!" })))
    } else {
        Ok(json(&json!({ "success": false, "message": "Sai tên đăng nhập hoặc mật khẩu!" })))
    }
}

// 👉 Hiển thị trang đăng nhập (GET /login)
pub fn login_page() -> String {
    r#"
    <html>
    <head>
        <title>Đăng Nhập</title>
        <link rel="stylesheet" href="/static/login.css">
        <script>
            async function submitLogin(event) {
                event.preventDefault();

                let formData = new FormData(document.getElementById("loginForm"));
                let response = await fetch("/login", {
                    method: "POST",
                    body: new URLSearchParams(formData)
                });

                let result = await response.json();
                let messageBox = document.getElementById("message");

                if (result.success) {
                    messageBox.style.background = "rgba(98, 177, 14, 0.9)"; // Xanh lá
                    messageBox.style.color = "white";
                    messageBox.innerHTML = result.message;
                    messageBox.style.display = "block";
                    setTimeout(() => { window.location.href = "/nghidinh"; }, 2000);
                } else {
                    messageBox.style.background = "rgba(255, 0, 0, 0.9)"; // Đỏ
                    messageBox.style.color = "white";
                    messageBox.innerHTML = result.message;
                    messageBox.style.display = "block";
                }
            }
        </script>
    </head>
    <body>
        <div class="message-box" id="message"></div>
        <div class="container">
            <h2>Đăng Nhập</h2>
            <form id="loginForm" onsubmit="submitLogin(event)">
                <label for="username">Tên đăng nhập:</label>
                <input type="text" id="username" name="username" placeholder="Nhập tên..." required>

                <label for="password">Mật khẩu:</label>
                <input type="password" id="password" name="password" placeholder="Nhập mật khẩu..." required>

                <button type="submit">Đăng Nhập</button>
                <button type="button" onclick="window.location.href='/register';">Đăng Ký</button>

            </form>
        </div>
    </body>
</html>
    "#.to_string()
}
