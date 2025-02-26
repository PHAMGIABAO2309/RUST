pub fn home() -> String {
    // JavaScript để lấy dữ liệu từ API và hiển thị lên trang HTML
    let js_code = r#"
    document.addEventListener("DOMContentLoaded", function () {
        // Gửi yêu cầu đến API `/api/content` để lấy danh sách dữ liệu
        fetch("/api/content")
            .then(response => response.json())  // Chuyển đổi phản hồi sang JSON
            .then(data => {
                // Kiểm tra nếu API trả về danh sách JSON có ít nhất một phần tử
                if (Array.isArray(data) && data.length > 0) {
                    let contentHTML = ""; // Chuỗi chứa toàn bộ nội dung
                    data.forEach(item => {
                        contentHTML += `<h2>${item.title}</h2>`;  // Thêm tiêu đề từng bài
                        contentHTML += `<p>${item.content.replace(/\r\n/g, '<br><br>')}</p>`;  // Thêm nội dung bài
                        contentHTML += `<hr>`;  // Thêm đường phân cách giữa các bài
                    });
                    document.getElementById("content").innerHTML = contentHTML;
                } else {
                    // Nếu API không có dữ liệu, hiển thị thông báo "Không có dữ liệu"
                    document.getElementById("content").innerHTML = "<p>Không có dữ liệu</p>";
                }
            })
            // Bắt lỗi nếu có lỗi khi tải dữ liệu từ API
            .catch(error => console.error("Lỗi tải dữ liệu:", error));
    });
    "#;


    format!(
        r#"<!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Trang Poem</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css" rel="stylesheet" />
    <link rel="stylesheet" href="/static/index.css">
</head>
<body>
    <header class="bg-white shadow">
        <div class="container mx-auto flex justify-between items-center py-4 px-6">
            <div class="flex items-center">
                <img alt="LuatVietnam logo" class="h-10" height="300" width="50" src="https://th.bing.com/th/id/OIP.-_GKfmytz61oq-HnVMf0NQHaHa?rs=1&pid=ImgDetMain" />
                <span class="ml-2 text-sm text-gray-600">Tiện ích văn bản luật</span>
            </div>
            <div class="flex items-center space-x-4">
                <button class="text-gray-600"></button>
                <span class="text-red-600"></span>
                <button class="text-gray-600"></button>
                <button class="text-gray-600"></button>
                <button class="text-gray-600">Đăng ký / Đăng nhập</button>
            </div>
        </div>
    </header>
        <main class="container mx-auto py-6 px-6">
        <div>
            <div class="flex items-center">
                <input class="border border-gray-300 rounded-l px-4 py-2" style="width: 600px;" placeholder="Tìm kiếm văn bản..." type="text"/>
                <button class="bg-red-600 text-white px-4 py-2 rounded-r">Tìm kiếm</button>
            </div>
            <h1 class="text-2xl font-bold mt-2">Nghị định 30/2020/NĐ-CP về công tác văn thư</h1>
            <div class="flex space-x-4 mt-4">
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Tóm tắt</button>
                <button class="bg-yellow-500 text-white px-4 py-2 rounded">Nội dung</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">VB gốc</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Tiếng Anh</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Hiệu lực</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">VB liên quan</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Lược đồ</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Nội dung MIX</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Tải về</button>
            </div>
            <div class="flex space-x-2 mt-2">
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Mục lục</button>
                <button class="bg-red-600 text-white px-4 py-2 rounded">So sánh VB cũ/mới</button>
                <button class="bg-red-600 text-white px-4 py-2 rounded">VB Song ngữ</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Tải về</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">VB Lưu</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Theo dõi VB</button>
                <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Ghi chú</button>
            </div>
            <div class="bg-white p-4 mt-4 shadow rounded w-[1000px] h-[430px] overflow-y-auto no-scrollbar ">
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
                <h1 id="title">Đang tải...</h1>
                <p id="content">Vui lòng chờ...</p>
            </div>
        </div>
    </main>
    <script>
        {js_code}
    </script>
</body>
</html>"#
    )
}