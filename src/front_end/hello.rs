pub fn home() -> String {
    let js_code = r#"
    async function loadContent() {
        try {
            let response = await fetch("/hello");
            let data = await response.json();

            document.getElementById("title").innerText = data.title;
            document.getElementById("content").innerHTML = data.content.replace(/\r\n/g, "<br><br>");
        } catch (error) {
            console.error("Lỗi tải dữ liệu:", error);
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
    <title>Document</title>
    <script src="https://cdn.tailwindcss.com"></script>
    
   
    <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css" rel="stylesheet" />
    <link rel="stylesheet" href="/static/index.css" />
</head>
<body class="bg-gray-100">
    <main class="container mx-auto py-6 px-6">
        <h2 id="title" class="text-center font-bold text-2xl"></h2>
        <div id="content" class="content-section"></div>
    </main>
    <script>
        {js_code}
    </script>
</body>
</html>"#,
    )
}
