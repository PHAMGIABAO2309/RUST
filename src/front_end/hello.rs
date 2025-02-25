

use sqlx::{ MySqlPool, Row};

pub async fn get_document_content(pool: &MySqlPool, chapter_name: &str) -> Result<String, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT d.Chapter, d.Title, ct.Rules, ct.Content
        FROM documents d
        INNER JOIN content_documents ct ON d.Chapter = ct.Rules
        WHERE d.Chapter = ?
        "#
    )
    .bind(chapter_name)
    .fetch_one(pool)
    .await?;
    let chapter: String = row.get("Chapter");
    let title: String = row.get("Title");
    let rules: String = row.get("Rules");
    let content: String = row.get("Content");

    Ok(document_content(&chapter, &title, &rules, &content))
}
pub fn document_content(chapter: &str, title: &str, rules: &str, content: &str) -> String {
    format!(
        r#"
    <div class="bg-white p-4 mt-4 shadow w-[1000px] h-[430px] overflow-y-auto no-scrollbar ">
        <div class="header-decree">
            <div>
                <p>CHÍNH PHỦ</p>
                <p>______</p>
                <p class="doc-number">Số: 30/2020/NĐ-CP</p>
            </div>
            <div>
                <p>CỘNG HÒA XÃ HỘI CHỦ NGHĨA VIỆT NAM</p>
                <p class="header-title">Độc lập - Tự do - Hạnh phúc</p>
                <span class="underline"></span>
                <p class="date">Hà Nội, ngày 05 tháng 3 năm 2020</p>
            </div>
        </div>
        <div class="content-section">
            <h2>Chương: {}</h2>
            <h3>Tiêu đề: {}</h3>
            <h4>Điều khoản: {}</h4>
            <p>Nội dung: {}</p>
        </div>
    </div>
    "#, chapter, title, rules, content
    )
}
pub fn home(chapter: &str, title: &str, rules: &str, content: &str) -> String {
    let document_html = document_content(chapter, title, rules, content);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta content="width=device-width, initial-scale=1.0" name="viewport" />
    <title>Document</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css" rel="stylesheet" />
    <link rel="stylesheet" href="/static/index.css" />
</head>
<body class="bg-gray-100">
    <header class="bg-white shadow">
    </header>
    <main class="container mx-auto py-6 px-6">
        <div>
            <div class="mt-4 bg-white p-4 shadow rounded">
                {document_html}
            </div>
        </div>
    </main>
</body>
</html>"#,
    )
}
