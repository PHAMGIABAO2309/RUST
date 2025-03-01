use sqlx::{mysql::MySqlPool, Row};
use warp::{Reply, Rejection};
use serde_json::{Value, Map};

use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;

const DB_URL: &str = "mysql://root@localhost:3366/kholuutruvanban";

// Kết nối tới database (trả về Arc<Pool<MySql>> để có thể clone)
pub async fn connect_db() -> Result<Arc<MySqlPool>, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .connect(DB_URL)
        .await?;
    Ok(Arc::new(pool))
}

/// Lấy dữ liệu từ tất cả các bảng trong database
pub async fn get_all_tables(db: Arc<MySqlPool>) -> Result<impl Reply, Rejection> {
    let tables = vec![
        "TaiKhoan", "ChucVu", "CoQuan", "LoaiVanBan", 
        "HoSo", "ChuKy", "NgonNgu", "ThongTinVanBan"
    ];

    let mut response = Map::new();

    for table in tables.iter() {
        let query = format!("SELECT * FROM {}", table);

        // Lấy schema của bảng
        let describe_query = format!("DESCRIBE {}", table);
        let column_names: Vec<String> = match sqlx::query(&describe_query)
            .fetch_all(&*db) // Sử dụng `&*db` để giải Arc
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
                        let value: Value = match row.try_get::<Option<String>, _>(i) {
                            Ok(Some(val)) => Value::String(val),
                            Ok(None) => Value::Null,
                            Err(_) => Value::String("Lỗi đọc dữ liệu".to_string()),
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
