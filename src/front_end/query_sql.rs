use sqlx::{MySqlPool, Row};
use serde_json::json;

pub async fn get_sql(pool: &MySqlPool) -> Result<serde_json::Value, sqlx::Error> {
    let document_rows = sqlx::query("SELECT CodeNumber, FileCatalog, Receives, Subject, ValidityStatus FROM infomation_documents_out;")
        .fetch_all(pool)
        .await?;
    let summary_rows = sqlx::query(
        "SELECT org.OranName, doc.CodeNumber, doc.FileCatalog, f.FileNonation, doc.CodeNotation, typeDoc.TypeName, acc.FullName, f.StartDate, f.EndDate
        FROM organization org
        JOIN account acc ON org.ID = acc.ID
        JOIN Files f ON org.OranId = f.OranId
        JOIN infomation_documents_out doc ON f.FileCode = doc.FileCode
        JOIN type_documents typeDoc ON doc.TypeId = typeDoc.TypeId
        WHERE typeDoc.OranId = org.OranId;"
    )
    .fetch_all(pool)
    .await?;

    let summary_content = sqlx::query("SELECT content FROM summary;")
    .fetch_all(pool)
    .await?;

    let documents: Vec<_> = document_rows.into_iter()
        .map(|row| {
            json!({
                "codenumber": row.get::<String, _>("CodeNumber"),
                "filecatalog": row.get::<String, _>("FileCatalog"),
                "subject": row.get::<String, _>("Subject"),
                "receives": row.get::<String, _>("Receives"),
                "validitystatus": row.get::<String, _>("ValidityStatus"),
            })
        })
        .collect();

        let summaries: Vec<_> = summary_rows.into_iter()
        .map(|row| {
            json!({
                "oran_name": row.get::<String, _>("OranName"),
                "code_number": row.get::<String, _>("CodeNumber"),
                "file_catalog": row.get::<String, _>("FileCatalog"),
                "file_nonation": row.get::<String, _>("FileNonation"),
                "code_notation": row.get::<String, _>("CodeNotation"),
                "type_name": row.get::<String, _>("TypeName"),
                "full_name": row.get::<String, _>("FullName"),
                "start_date": row.get::<Option<chrono::NaiveDate>, _>("StartDate")
                    .map(|d| d.to_string())
                    .unwrap_or("N/A".to_string()),  // Xử lý NULL
                "end_date": row.get::<Option<chrono::NaiveDate>, _>("EndDate")
                    .map(|d| d.to_string())
                    .unwrap_or("N/A".to_string()),  // Xử lý NULL
            })
        })
        .collect();

        let summary_content: Vec<_> = summary_content.into_iter()
        .map(|row| {
            json!({
                "content": row.get::<String, _>("content"),
            })
        })
        .collect();
    

    Ok(json!({
        "documents": documents,
        "summary": summaries,
        "summary_content": summary_content,
    }))
}

