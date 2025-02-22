use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;

// Định nghĩa URL cố định
const DB_URL: &str = "mysql://root@localhost:3366/my_database";

// Kết nối tới database (trả về Pool<MySql>)
pub async fn connect_db() -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPoolOptions::new()
        .connect(DB_URL)
        .await
}
// Lấy nội dung thơ từ database
pub async fn get_poem_content(pool: &Pool<MySql>) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("SELECT content FROM poems LIMIT 1")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn register_user(pool: &Pool<MySql>, username: &str, password: &str) -> Result<(), sqlx::Error> {
    let result = sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(username)
        .bind(password)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Lỗi khi đăng ký: {:?}", e);
            Err(e)
        }
    }
}



