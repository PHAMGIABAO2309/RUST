use sqlx::{MySqlPool, Row};
use serde_json::json;

pub async fn get_document_content(pool: &MySqlPool) -> Result<serde_json::Value, sqlx::Error> {
    let rows = sqlx::query("SELECT SoVanBan, NamHinhThanh, TieuDe, TrichYeuNoiDung, NoiDung FROM thongtinvanban;")
        .fetch_all(pool)
        .await?;

    let documents: Vec<_> = rows.into_iter()
        .map(|row| {
            let sovanban: String = row.get("SoVanBan");
            let namhinhthanh: String = row.get("NamHinhThanh");
            let title: String = row.get("TieuDe");
            let trichyeu: String = row.get("TrichYeuNoiDung");
            let content: String = row.get("NoiDung");

            json!({
                "sovanban": sovanban,
                "namhinhthanh": namhinhthanh,
                "title": title,
                "trichyeu": trichyeu,
                "content": content
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
                if (Array.isArray(data) && data.length > 0)
                {
                    document.getElementById("sovanban").innerText = data[0].sovanban;
                    document.getElementById("namhinhthanh").innerText = data[0].namhinhthanh;
                    let contentHTML = "";
                    data.forEach(item => 
                    {
                        contentHTML += `<br><br><h2 style="font-weight: bold; text-align: center;">${item.title}</h2><br><br>`;
                        contentHTML += `<p>${item.trichyeu.replace(/\r\n/g, '<br>')}</p><br>`;
                        
                        // Bôi đậm "Điều X."
                        let formattedContent = item.content.replace(/\r\n/g, '<br><br>');
                        formattedContent = formattedContent.replace(/(Mục .*?)<br>/g, '<h2 style="text-align: center;"><strong>$1</strong></h2>');
                        formattedContent = formattedContent.replace(/(Chương .*?)<br>/g, '<h2 style="text-align: center;"><strong>$1</strong></h2><br>');
                        formattedContent = formattedContent.replace(/(Điều \d+\..*?)<br><br>/g, '<strong>$1</strong><br><br>');

                        contentHTML += `<p>${formattedContent}</p>`;
                    });
                    document.getElementById("content").innerHTML = contentHTML;
                }
                else
                {
                    document.getElementById("content").innerHTML = "<p>Không có dữ liệu</p>";
                }
            })
            .catch(error => console.error("Lỗi tải dữ liệu:", error));
    });
    "#.to_string()
}
