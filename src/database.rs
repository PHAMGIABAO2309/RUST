use sqlx::{MySql, Pool};

const DB_URL: &str = "mysql://root@localhost:3366/my_database"; // Định nghĩa URL cố định

pub async fn get_poem_content() -> String {
    let pool = Pool::<MySql>::connect(DB_URL)
        .await
        .expect("Không thể kết nối tới MySQL");

    let row: (String,) = sqlx::query_as("SELECT content FROM poems LIMIT 1")
        .fetch_one(&pool)
        .await
        .expect("Không thể lấy dữ liệu từ MySQL");

    row.0
}
