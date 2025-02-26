pub fn home() -> String {
    let js_code = r#"
    document.addEventListener("DOMContentLoaded", function () {
        fetch("/api/content")
            .then(response => response.json())
            .then(data => {
                document.getElementById("title").innerText = data.title;
                document.getElementById("content").innerHTML = data.content.replace(/\r\n/g, '<br><br>');
            })
            .catch(error => console.error("Lỗi tải dữ liệu:", error));
    });
    "#;

    format!(
        r#"<!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Trang Poem</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <h1 id="title">Đang tải...</h1>
    <p id="content">Vui lòng chờ...</p>

    <script>
        {js_code}
    </script>
</body>
</html>"#
    )
}
