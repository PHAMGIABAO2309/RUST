use sqlx::{mysql::MySqlPool, Row};
use warp::{Reply, Rejection};
use serde_json::{Value, Map};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;
use chrono::NaiveDate;

const DB_URL: &str = "mysql://root@localhost/storages_documents";

// Kết nối tới database
pub async fn connect_db() -> Result<Arc<MySqlPool>, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .connect(DB_URL)
        .await?;
    Ok(Arc::new(pool))
}

/// Lấy dữ liệu từ tất cả các bảng trong database
pub async fn get_all_tables(db: Arc<MySqlPool>) -> Result<impl Reply, Rejection> {
    let tables = vec![
        "account", "files", "languages", "organization", 
        "positions", "signatures", "type_documents", "infomation_documents_out", "infomation_documents_arrival"
    ];

    let mut response = Map::new();

    for table in tables.iter() {
        let query = format!("SELECT * FROM {}", table);

        // Lấy schema của bảng
        let describe_query = format!("DESCRIBE {}", table);
        let column_names: Vec<String> = match sqlx::query(&describe_query)
            .fetch_all(&*db)
            .await
        {
            Ok(columns) => columns
                .iter()
                .map(|row| row.get::<String, _>("Field"))
                .collect(),
            Err(_) => {
                response.insert(table.to_string(), Value::String("Lỗi lấy schema".to_string()));
                continue;
            }
        };

        match sqlx::query(&query).fetch_all(&*db).await {
            Ok(rows) => {
                let mut result: Vec<Value> = Vec::new();
                
                for row in rows {
                    let mut json_row = Map::new();

                    for (i, column_name) in column_names.iter().enumerate() {
                        let value: Value = if column_name.to_lowercase().contains("date") {
                            // Xử lý cột DATE
                            match row.try_get::<Option<NaiveDate>, _>(i) {
                                Ok(Some(date)) => Value::String(date.format("%Y-%m-%d").to_string()), // Chuyển thành chuỗi
                                Ok(None) => Value::Null,
                                Err(_) => Value::String("Lỗi đọc ngày".to_string()),
                            }
                        } else {
                            // Xử lý các cột khác
                            match row.try_get::<Option<String>, _>(i) {
                                Ok(Some(val)) => Value::String(val),
                                Ok(None) => Value::Null,
                                Err(_) => Value::String("Lỗi đọc dữ liệu".to_string()),
                            }
                        };

                        json_row.insert(column_name.clone(), value);
                    }
                    result.push(Value::Object(json_row));
                }

                response.insert(table.to_string(), Value::Array(result));
            }
            Err(err) => {
                response.insert(table.to_string(), Value::String(format!("Lỗi truy vấn: {}", err)));
            }
        }
    }

    Ok(warp::reply::json(&response))
}
