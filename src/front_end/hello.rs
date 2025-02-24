pub fn home(poem_content: &str) -> String {
  format!(
      r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>Document</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css"/>
</head>
<body class="bg-gray-100">
  <header class="bg-white shadow">
      <div class="container mx-auto flex justify-between items-center py-4 px-6">
          <div class="flex items-center">
              <img src="https://storage.googleapis.com/a1aa/image/1id_n8_P-1fCXoyPjLLKZzD48YWK4FDjAV34G1wRj_k.jpg" 
                   alt="LuatVietnam logo" class="h-10" width="150" height="50"/>
              <span class="ml-2 text-sm text-gray-600">Tiện ích văn bản luật</span>
          </div>
          <div class="flex items-center space-x-4">
              <button class="text-gray-600">Danh mục</button>
              <span class="text-red-600">Tổng đài trực tuyến 19006192</span>
              <button class="text-gray-600">Gói dịch vụ &amp; Giá</button>
              <button class="text-gray-600">Tiếng Anh</button>
              <button class="text-gray-600">Đăng ký / Đăng nhập</button>
          </div>
      </div>
  </header>
  
  <main class="container mx-auto py-6 px-6">
      <h1 class="text-2xl font-bold mt-6">Nghị định 30/2020/NĐ-CP về công tác văn thư</h1>
      <div class="bg-white p-4 mt-4 shadow">
          <h2 class="text-xl font-bold">CHÍNH PHỦ</h2>
          <p class="mt-2">Số: 30/2020/NĐ-CP</p>
          <p class="mt-2">Hà Nội, ngày 05 tháng 3 năm 2020</p>
          <img src="https://storage.googleapis.com/a1aa/image/heGjcg_7YbPP9Mcf-3wHvkQQ2k8FT_rDkc1k3NmthGk.jpg" 
               alt="Official stamp" class="mt-4" width="200" height="100"/>
      </div>
      
      <div class="mt-6">
          {poem_content}
      </div>
  </main>
</body>
</html>"#
  )
}
