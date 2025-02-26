use sqlx::{MySqlPool, Row};

pub async fn get_document_content(pool: &MySqlPool) -> Result<String, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT Title, Content FROM documents;
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut html_content = String::new();

    for row in rows {
        let title: String = row.get("Title");
        let content: String = row.get("Content");

        html_content.push_str(&document_content(&title, &content));
    }

    Ok(html_content)
}

pub fn document_content(title: &str, content: &str) -> String {
    format!(
        r#"
    <div class="content-section">
        <h2>Chương: {}</h2>
        <p>Tiêu đề: {}</p>
    </div>
    "#,
        title.replace("\n", "<br>"),
        content.replace("\n", "<br>")
    )
}
