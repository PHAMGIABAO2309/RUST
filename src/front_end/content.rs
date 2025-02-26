use sqlx::{MySqlPool, Row};
use regex::Regex;

pub async fn get_document_content(pool: &MySqlPool) -> Result<String, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT Title, Content FROM documents;
        "#,
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
        <h2 style="font-weight: bold; text-align: center;">{}</h2>
        <p>{}</p>
    </div>
    "#,
        title.replace("\r\n", "<br>"),
        formatted_content
    )
}
