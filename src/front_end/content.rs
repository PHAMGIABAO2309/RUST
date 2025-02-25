use sqlx::{ MySqlPool, Row};

pub async fn get_document_content(pool: &MySqlPool, chapter_name: &str) -> Result<String, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT d.Chapter, d.Title,  ct.Content
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
    <div class="content-section">
        <h2>Chương: {}</h2>
        <h3>Tiêu đề: {}</h3>
        <h4>Điều khoản: {}</h4>
        <p>Nội dung: {}</p>
    </div>
    "#, chapter, title, rules, content
    )
}