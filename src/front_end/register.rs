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
use warp::reply::json;
use serde_json::json;

pub async fn handle_register(pool: MySqlPool, form: RegisterForm) -> Result<impl Reply, Rejection> {
    if username_exists(&pool, &form.username).await.unwrap_or(false) {
        return Ok(json(&json!({ "success": false, "message": "Tên đăng nhập đã tồn tại!" })));
    }

    let query = "INSERT INTO users (username, password, email) VALUES (?, ?, ?)";
    match sqlx::query(query)
        .bind(&form.username)
        .bind(&form.password)
        .bind(&form.email)
        .execute(&pool)
        .await 
    {
        Ok(_) => Ok(json(&json!({ "success": true }))),
        Err(_) => Ok(json(&json!({ "success": false, "message": "Lỗi khi đăng ký!" }))),
    }
}

// 👉 Hiển thị trang đăng ký (GET /register)
pub fn register_page() -> String {
    r#"
    <html>
    <head>
        <title>Đăng Ký</title>
        <link rel="stylesheet" href="/static/register.css">
    </head>
    <body>
        <div class="container">
            <h2>Đăng Ký Tài Khoản</h2>
            <form id="register-form" action="/register" method="post">
                <label for="username">Tên đăng nhập:</label>
                <input type="text" id="username" name="username" placeholder="Nhập tên..." required>
                
                <label for="password">Mật khẩu:</label>
                <input type="password" id="password" name="password" placeholder="Nhập mật khẩu..." required>
                
                <label for="email">Email:</label>
                <input type="email" id="email" name="email" placeholder="Nhập email..." required>
                
                <button type="submit">Đăng Ký</button>
            </form>
        </div>

        <!-- Toast Notification -->
        <div class="toast-container"></div>

        <script>
            function showToast(message, type = "success", redirect = null) {
                const toastContainer = document.querySelector(".toast-container");
                const toast = document.createElement("div");
                toast.className = "toast " + (type === "error" ? "error" : "success");
                toast.textContent = message;
                toastContainer.appendChild(toast);
                
                setTimeout(() => toast.classList.add("show"), 100);
                setTimeout(() => {
                    toast.classList.remove("show");
                    setTimeout(() => toast.remove(), 500);
                    if (redirect) window.location.href = redirect;
                }, 2000);
            }

            // Xử lý submit form qua fetch để không cần tải lại trang
            document.getElementById("register-form").addEventListener("submit", function(event) {
                event.preventDefault(); // Ngăn form gửi đi theo cách thông thường
                
                const formData = new FormData(this);
                fetch("/register", {
                    method: "POST",
                    body: formData
                })
                .then(response => response.json())
                .then(data => {
                    if (data.success) {
                        showToast("Đăng ký thành công!", "success", "/hello");
                    } else {
                        showToast("Đăng ký thất bại! " + (data.message || ""), "error");
                    }
                })
                .catch(() => showToast("Lỗi kết nối!", "error"));
            });
        </script>
    </body>
</html>
    "#.to_string()
}



