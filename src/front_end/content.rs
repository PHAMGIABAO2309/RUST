use sqlx::{MySql, Pool};

pub async fn get_poem_content(pool: &Pool<MySql>) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("SELECT Rules FROM content_documents LIMIT 1")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub fn document_content(poem_content: &str) -> String {
    format!(
        r#"
    <div class="bg-white p-4 mt-4 shadow w-[1000px] h-[430px] overflow-y-auto no-scrollbar ">
        <div class="header-decree">
            <div>
                <p>CHÍNH PHỦ</p>
                <p>______</p>
                <p class="doc-number">Số: 30/2020/NĐ-CP</p>
            </div>
            <div>
                <p>CỘNG HÒA XÃ HỘI CHỦ NGHĨA VIỆT NAM</p>
                <p class="header-title">Độc lập - Tự do - Hạnh phúc</p>
                <span class="underline"></span>
                <p class="date">Hà Nội, ngày 05 tháng 3 năm 2020</p>
            </div>
        </div>
        <p>{}</p>
    </div>
    "#,
        poem_content
    )
}
