use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;

const DB_URL: &str = "mysql://root@localhost:3366/kholuutruvanban";

// Kết nối tới database (trả về Pool<MySql>)
pub async fn connect_db() -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPoolOptions::new()
        .connect(DB_URL)
        .await
}
// Lấy nội dung thơ từ database






