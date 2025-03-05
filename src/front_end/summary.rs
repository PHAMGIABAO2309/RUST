use reqwest;
use sqlx::{MySqlPool, Row};
use serde_json::json;

use serde_json::Value;
use std::error::Error;

pub async fn fetch_summary() -> Result<Value, Box<dyn Error>> {
    let url = "http://localhost:9090/api/all";  
    let response = reqwest::get(url).await?.json::<Value>().await?;
    Ok(response)
}


pub async fn get_summary(pool: &MySqlPool) -> Result<serde_json::Value, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT 
            org.OranName, 
            doc.CodeNumber, 
            doc.FileCatalog, 
            doc.CodeNotation, 
            typeDoc.TypeName, 
            acc.FullName, 
            f.StartDate,
            f.EndDate
        FROM organization org
        JOIN account acc ON org.ID = acc.ID
        JOIN Files f ON org.OranId = f.OranId
        JOIN infomation_documents_out doc ON f.FileCode = doc.FileCode
        JOIN type_documents typeDoc ON doc.TypeId = typeDoc.TypeId
        WHERE typeDoc.OranId = org.OranId;"
    )
    .fetch_all(pool)
    .await?;

    let documents: Vec<_> = rows
        .into_iter()
        .map(|row| {
            json!({
                "oran_name": row.get::<String, _>("OranName"),
                "code_number": row.get::<String, _>("CodeNumber"),
                "file_catalog": row.get::<String, _>("FileCatalog"),
                "code_notation": row.get::<String, _>("CodeNotation"),
                "type_name": row.get::<String, _>("TypeName"),
                "full_name": row.get::<String, _>("FullName"),
                "start_date": row.get::<chrono::NaiveDate, _>("StartDate").to_string(),
                "end_date": row.get::<chrono::NaiveDate, _>("EndDate").to_string(),
            })
        })
        .collect();

    Ok(json!(documents))
}

pub fn get_js_code () -> String {
    r#"
        document.addEventListener("DOMContentLoaded", function () {
            fetch("http://localhost:8080/summary")
            .then(response => response.json())
            .then(data => {
                if (Array.isArray(data) && data.length > 0) {
                    const firstDoc = data[0];

                    if (firstDoc.oran_name) {
                        let tencoquanEl = document.getElementById("chinhphu");
                        if (tencoquanEl) tencoquanEl.innerText = firstDoc.oran_name;
                    }
                }
            })
            .catch(error => console.error("Error fetching data:", error));
        });
    "#.to_string()
}



pub fn summary() -> String {
    let js_code = get_js_code();
    format! (
        r#"
        <div class="change page hidden" id="page_two">
            <div class="bg-gray p-1 mt-4 pl-4 shadow rounded w-[850px] h-[70px]">
                <div class="flex space-x-2 mt-2">
                    <button class="bg-red-600 text-white px-4 py-2 rounded     item">VB Song ngữ</button>
                    <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded item">Tải VB</button>
                    <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded item"> Lưu</button>
                    <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded item"> Theo dõi VB</button>
                    <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded item"> Báo lỗi</button>
                </div>
            </div>
            <div class=" bg-white p-4 mt-4 shadow rounded w-[850px] h-[430px] overflow-y-auto no-scrollbar " >
                <div>
                    <h1 class="text-xl font-bold mb-2">THUỘC TÍNH NGHỊ ĐỊNH 30/2020/NĐ-CP</h1>
                    <p class="mb-4">Nghị định 30/2020/NĐ-CP của Chính phủ về công tác văn thư</p>
                    <div class="overflow-x-auto">
                        <table class="min-w-full bg-white border border-gray-300">
                            <tbody>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Cơ quan ban hành:</td>
                                    <td class="px-4 py-2 border-r" id="chinhphu"></td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Số công báo:</td>
                                    <td class="px-4 py-2 text-red-500">Đã biết <i class="fas fa-info-circle"></i></td>
                                </tr>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Số hiệu:</td>
                                    <td class="px-4 py-2 border-r">30/2020/NĐ-CP</td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Ngày đăng công báo:</td>
                                    <td class="px-4 py-2 text-red-500">Đã biết <i class="fas fa-info-circle"></i></td>
                                </tr>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Loại văn bản:</td>
                                    <td class="px-4 py-2 border-r">Nghị định</td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Người ký:</td>
                                    <td class="px-4 py-2">Nguyễn Xuân Phúc</td>
                                </tr>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Ngày ban hành:</td>
                                    <td class="px-4 py-2 border-r">05/03/2020</td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Ngày hết hiệu lực:</td>
                                    <td class="px-4 py-2 text-gray-500">Đang cập nhật</td>
                                </tr>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Áp dụng:</td>
                                    <td class="px-4 py-2 text-red-500 border-r">Đã biết <i class="fas fa-info-circle"></i></td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Tình trạng hiệu lực:</td>
                                    <td class="px-4 py-2 text-red-500">Đã biết <i class="fas fa-info-circle"></i></td>
                                </tr>
                                <tr>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Lĩnh vực:</td>
                                    <td class="px-4 py-2 border-r">Hành chính</td>
                                    <td class="px-4 py-2 border-r bg-gray-100"></td>
                                    <td class="px-4 py-2"></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>

        </div>
        <script>
            {js_code}
        </script>
        "#
    )
}