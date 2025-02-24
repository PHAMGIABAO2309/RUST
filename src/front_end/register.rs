use sqlx::MySqlPool;
use serde::Deserialize;
use warp::{Rejection, Reply};
use warp::reply::json;
use serde_json::json;
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
// Hàm kiểm tra email đã tồn tại chưa
pub async fn email_exists(pool: &MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
    let query = "SELECT COUNT(*) FROM users WHERE email = ?";
    let count: (i64,) = sqlx::query_as(query)
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(count.0 > 0)
}

// 👉 Xử lý đăng ký nhiều tài khoản (POST /register)
pub async fn handle_register(pool: MySqlPool, form: RegisterForm) -> Result<impl Reply, Rejection> {
    // Kiểm tra trùng tên đăng nhập
    if username_exists(&pool, &form.username).await.unwrap_or(false) {
        return Ok(json(&json!({ "success": false, "message": "Tên đăng nhập đã tồn tại, vui lòng chọn tên khác!" })));
    }
    
    // Kiểm tra trùng email
    if email_exists(&pool, &form.email).await.unwrap_or(false) {
        return Ok(json(&json!({ "success": false, "message": "Email đã tồn tại, vui lòng sử dụng email khác!" })));
    }

    let query = "INSERT INTO users (username, password, email) VALUES (?, ?, ?)";
    match sqlx::query(query)
        .bind(&form.username)
        .bind(&form.password)
        .bind(&form.email)
        .execute(&pool)
        .await 
    {
        Ok(_) => Ok(json(&json!({ "success": true, "message": "Đăng ký thành công!" }))),
        Err(e) => {
            eprintln!("Lỗi khi đăng ký: {:?}", e);
            Ok(json(&json!({ "success": false, "message": "Đăng ký thất bại, thử lại!" })))
        }
    }
}
// 👉 Hiển thị trang đăng ký (GET /register)
pub fn register_page() -> String {
    r#"
    <html>
    <head>
        <title>Đăng Ký</title>
        <link rel="stylesheet" href="/static/register.css">
        <script>
            async function submitForm(event) {
                event.preventDefault(); // Ngăn trang tải lại

                let formData = new FormData(document.getElementById("registerForm"));
                let response = await fetch("/register", {
                    method: "POST",
                    body: new URLSearchParams(formData)
                });

                let result = await response.json();
                let messageBox = document.getElementById("message");

                if (result.success) {
                    messageBox.style.background = "rgba(98, 177, 14, 0.9)"; // Xanh lá
                    messageBox.style.color = "white";
                    messageBox.innerHTML = result.message;
                    messageBox.style.display = "block"; // Hiện thông báo
                    setTimeout(() => { window.location.href = "/login"; }, 2000);
                } else {
                    messageBox.style.background = "rgba(255, 0, 0, 0.9)"; // Đỏ
                    messageBox.style.color = "white";
                    messageBox.innerHTML = result.message;
                    messageBox.style.display = "block"; // Hiện thông báo
                }
            }
        </script>
    </head>
    <body>
        <div class="message-box" id="message"></div>
        <div class="container">
            <h2>Đăng Ký Tài Khoản</h2>
            <form id="registerForm" onsubmit="submitForm(event)">
                <label for="username">Tên đăng nhập:</label>
                <input type="text" id="username" name="username" placeholder="Nhập tên..." required>

                <label for="password">Mật khẩu:</label>
                <input type="password" id="password" name="password" placeholder="Nhập mật khẩu..." required>

                <label for="email">Email:</label>
                <input type="email" id="email" name="email" placeholder="Nhập email..." required>

                <button type="submit">Đăng Ký</button>
            </form>
        </div>
    </body>
</html>
    "#.to_string()
}



