use sqlx::{MySqlPool, Row};
use serde_json::json;

pub async fn get_document_content(pool: &MySqlPool) -> Result<serde_json::Value, sqlx::Error> {
    let rows = sqlx::query("SELECT CodeNumber, FileCatalog, Subject FROM infomation_documents_out;")
        .fetch_all(pool)
        .await?;

    let documents: Vec<_> = rows.into_iter()
        .map(|row| {
            let codenumber: String = row.get("CodeNumber");
            let filecatalog: String = row.get("FileCatalog");
            let subject: String = row.get("Subject");

            json!({
                "codenumber": codenumber,
                "filecatalog": filecatalog,
                "subject": subject,
            })
        })
        .collect();

    Ok(json!(documents))
}
pub fn get_js_code() -> String {
    r#"
    document.addEventListener("DOMContentLoaded", function () {
    fetch("/api/content")
        .then(response => response.json())
        .then(data => {
            if (Array.isArray(data) && data.length > 0) {
                document.getElementById("sovanban").innerText = data[0].codenumber;
                document.getElementById("namhinhthanh").innerText = data[0].filecatalog;
                
                let contentHTML = "";
                data.forEach(item => {
                    contentHTML += `<br><br><h2 style="font-weight: bold; text-align: center;">${item.codenumber}</h2><br><br>`;
                    
                    let formattedContent = item.subject.replace(/\r\n/g, '<br><br>');
                    formattedContent = formattedContent.replace(/(Mục .*?)<br>/g, '<h2 style="text-align: center;"><strong>$1</strong></h2>');
                    formattedContent = formattedContent.replace(/(Chương .*?)<br>/g, '<h2 style="text-align: center;"><strong>$1</strong></h2><br>');
                    formattedContent = formattedContent.replace(/(Điều \d+\..*?)<br><br>/g, '<strong>$1</strong><br><br>');

                    contentHTML += `<p>${formattedContent}</p>`;
                });

                document.getElementById("content").innerHTML = contentHTML;
            } else {
                document.getElementById("content").innerHTML = "<p>Không có dữ liệu</p>";
            }
        })
        .catch(error => console.error("Lỗi tải dữ liệu:", error));
});

    "#.to_string()
}
