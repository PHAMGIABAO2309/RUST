use sqlx::{MySqlPool, Row};
use regex::Regex;

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
    let bold_regex = Regex::new(r"(Điều \d+\..*?)\n").unwrap();
    let formatted_content = bold_regex.replace_all(content, |caps: &regex::Captures| {
        format!("<strong>{}</strong>", &caps[1])
    });

    format!(
        r#"
    <div class="content-section">
        <h2 style="font-weight: bold; text-align: center;">{}</h2>
        <p>{}</p>
    </div>
    "#,
        title.replace("\n", "<br>"),
        formatted_content.replace("\n", "<br>")
    )
}
