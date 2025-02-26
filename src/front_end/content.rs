use sqlx::{MySqlPool, Row};
use regex::Regex;
use serde_json::json;

pub async fn get_document_content(pool: &MySqlPool) -> Result<serde_json::Value, sqlx::Error> {
    let rows = sqlx::query("SELECT Title, Content FROM documents;")
        .fetch_all(pool)
        .await?;
    
    let documents: Vec<_> = rows.into_iter()
        .map(|row| {
            let title: String = row.get("Title");
            let content: String = row.get("Content");
            json!({ "title": title, "content": content })
        })
        .collect();
    
    Ok(json!(documents))
}
pub fn document_content(title: &str, content: &str) -> String {
    let bold_regex = Regex::new(r"(Điều \d+\..*?)\r\n").unwrap();
     // Bôi đậm các dòng "Điều X."
    let formatted_content = bold_regex.replace_all(content, |caps: &regex::Captures| {
        format!("<strong>{}</strong><br><br>", &caps[1])
    });
    // Chuyển đổi xuống dòng `\r\n` thành `<br>` để hiển thị đúng trên HTML
    let formatted_content = formatted_content.replace("\r\n", "<br><br>");
    format!(
        r#"
    <div class="content-section">
        <h2 style="font-weight: bold; text-align: center;">{}</h2><br>
        <p>{}</p>
    </div>
    "#,
        title.replace("\r\n", "<br>"),
        formatted_content
    )
}
