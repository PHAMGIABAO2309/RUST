pub fn register_html() -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Đăng Ký</title>
    <link rel="stylesheet" href="/static/register.css">
</head>
<body>
    <div class="container">
        <form id="registerForm" class="register-form" method="post">
            <h2>Đăng Ký</h2>
            <input type="text" name="username" id="username" placeholder="Tên đăng nhập" required>
            <input type="email" name="email" id="email" placeholder="Email" required>
            <input type="password" name="password" id="password" placeholder="Mật khẩu" required>
            <button type="submit">Đăng Ký</button>
            <p id="message"></p>
        </form>
    </div>
    <script src="/static/register.js" defer></script>
</body>
</html>"#
    )
}
