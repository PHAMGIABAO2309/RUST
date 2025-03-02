use sqlx::{MySqlPool, Row};
use serde_json::json;

pub async fn get_document_content(pool: &MySqlPool) -> Result<serde_json::Value, sqlx::Error> {
    let rows = sqlx::query("SELECT CodeNumber, FileCatalog, Receives, Subject, ValidityStatus
    FROM infomation_documents_out;")
        .fetch_all(pool)
        .await?;

    let documents: Vec<_> = rows.into_iter()
        .map(|row| {
            let codenumber: String = row.get("CodeNumber");
            let filecatalog: String = row.get("FileCatalog");
            let subject: String = row.get("Subject");
            let receives: String = row.get("Receives");
            let validitystatus: String = row.get("ValidityStatus");

            json!({
                "codenumber": codenumber,
                "filecatalog": filecatalog,
                "subject": subject,
                "receives": receives,
                "validitystatus": validitystatus,
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
                    const firstDoc = data[0];

                    if (firstDoc.codenumber) {
                        let sovanbanEl = document.getElementById("sovanban");
                        if (sovanbanEl) sovanbanEl.innerText = firstDoc.codenumber;
                    }

                    // Lấy tất cả các phần tử có id="namhinhthanh"
                    let namhinhthanhElements = document.querySelectorAll("[id='namhinhthanh']");
                    namhinhthanhElements.forEach(el => {
                        if (firstDoc.filecatalog) el.innerText = firstDoc.filecatalog;
                    });

                    let noinhanElements = document.querySelectorAll("[id='noinhan']");
                    noinhanElements.forEach(el => {
                        if (firstDoc.receives) el.innerText = firstDoc.receives;
                    });

                    let hieulucElements = document.querySelectorAll("[id='validitystatus']");
                    hieulucElements.forEach(el => {
                        if (firstDoc.validitystatus) el.innerText = firstDoc.validitystatus;
                    });

                    let contentHTML = "";
                    data.forEach(item => {
                        if (item.subject) {
                            let formattedContent = item.subject.replace(/\r\n/g, '<br><br>');
                            formattedContent = formattedContent.replace(/(NGHỊ .*?)<br>/g, '<br><br><h2 style="text-align: center;"><strong>$1</strong></h2><br>');
                            formattedContent = formattedContent.replace(/(Mục .*?)<br>/g, '<h2 style="text-align: center;"><strong>$1</strong></h2>');
                            formattedContent = formattedContent.replace(/(Chương .*?)<br>/g, '<h2 style="text-align: center;"><strong>$1</strong></h2><br>');
                            formattedContent = formattedContent.replace(/(Điều \\d+\\..*?)<br><br>/g, '<strong>$1</strong><br><br>');
                            
                            contentHTML += `<p>${formattedContent}</p>`;
                        }
                    });

                    let contentEl = document.getElementById("content");
                    if (contentEl) {
                        contentEl.innerHTML = contentHTML;
                    }
                } else {
                    let contentEl = document.getElementById("content");
                    if (contentEl) contentEl.innerHTML = "<p>Không có dữ liệu</p>";
                }
            })
            .catch(error => console.error("Lỗi tải dữ liệu:", error));
    });
    "#.to_string()
}
