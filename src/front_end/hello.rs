pub fn home(title: String, content: String) -> String {
    let js_code = r#"
    async function loadContent() {
        try {
            let response = await fetch('/api/content');
            let data = await response.json();

            document.getElementById('title').innerText = data.title;
            document.getElementById('content').innerHTML = data.content.replace(/\r\n/g, '<br><br>');
        } catch (error) {
            console.error('Lỗi tải dữ liệu:', error);
        }
    }

    window.onload = loadContent;
    "#;

    format!(
        r#"<!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-100">
    <main class="container mx-auto py-6 px-6">
        <h2 id="title" class="text-center font-bold text-2xl">{title}</h2>
        <div id="content" class="content-section">{content}</div>
    </main>
    <script>
        {js_code}
    </script>
</body>
</html>"#,
        title = title,
        content = content
    )
}
