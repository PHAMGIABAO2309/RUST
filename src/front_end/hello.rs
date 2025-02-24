 // Thêm module content
 use crate::front_end::content:: get_document_content;
 use sqlx::MySqlPool;
 pub async fn home(pool: &MySqlPool, chapter_name: &str) -> Result<String, sqlx::Error> {
  let content = get_document_content(pool, chapter_name).await?;
  Ok(format!(
      r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta content="width=device-width, initial-scale=1.0" name="viewport" />
  <title>Document</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css" rel="stylesheet" />
  <link rel="stylesheet" href="/static/index.css" />
</head>
<body class="bg-gray-100">
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
      <div class="flex items-center ">
        <input class=" border border-gray-300 rounded-l px-4 py-2" style="width: 600px;" placeholder="Tìm kiếm văn bản..." type="text"/>
        <button class="bg-red-600 text-white px-4 py-2 rounded-r">Tìm kiếm</button>
      </div>
      <h1 class="text-2xl font-bold mt-2">Nghị định 30/2020/NĐ-CP về công tác văn thư</h1>
      <div class="flex space-x-4 mt-4">
        <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Tóm tắt</button>
        <button class="bg-yellow-500 text-white px-4 py-2 rounded"> Nội dung</button>
        <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded"> VB gốc</button>
        <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded">Tiếng Anh</button>
        <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded"> Hiệu lực</button>
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
      <div>
      {}
      </div>
    </div>
  </main>
</body>
</html>"#,
        content
  ))
}
